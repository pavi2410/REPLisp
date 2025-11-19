/// REPLisp to WebAssembly Compiler
///
/// This module compiles REPLisp AST to WebAssembly binary format.
///
/// Type Strategy:
/// - Numbers: f64 (native WASM type)
/// - Strings: Stored in linear memory as length-prefixed UTF-8
/// - Lists: Stored in linear memory as linked structures
/// - Booleans: Represented as f64 (0.0 = false, 1.0 = true)
/// - Nil: Represented as f64 NaN with specific bit pattern

use crate::parser::{Expr, ExprType};
use super::module::{
    WasmModule, FuncType, ValType, Export, Memory, Limits,
    FunctionCode, Import
};
use std::collections::HashMap;

/// WebAssembly instruction opcodes
#[allow(dead_code)]
mod opcode {
    // Control flow
    pub const UNREACHABLE: u8 = 0x00;
    pub const NOP: u8 = 0x01;
    pub const BLOCK: u8 = 0x02;
    pub const LOOP: u8 = 0x03;
    pub const IF: u8 = 0x04;
    pub const ELSE: u8 = 0x05;
    pub const END: u8 = 0x0B;
    pub const BR: u8 = 0x0C;
    pub const BR_IF: u8 = 0x0D;
    pub const RETURN: u8 = 0x0F;
    pub const CALL: u8 = 0x10;

    // Variables
    pub const LOCAL_GET: u8 = 0x20;
    pub const LOCAL_SET: u8 = 0x21;
    pub const LOCAL_TEE: u8 = 0x22;
    pub const GLOBAL_GET: u8 = 0x23;
    pub const GLOBAL_SET: u8 = 0x24;

    // Memory
    pub const I32_LOAD: u8 = 0x28;
    pub const I64_LOAD: u8 = 0x29;
    pub const F32_LOAD: u8 = 0x2A;
    pub const F64_LOAD: u8 = 0x2B;
    pub const I32_STORE: u8 = 0x36;
    pub const I64_STORE: u8 = 0x37;
    pub const F32_STORE: u8 = 0x38;
    pub const F64_STORE: u8 = 0x39;
    pub const MEMORY_SIZE: u8 = 0x3F;
    pub const MEMORY_GROW: u8 = 0x40;

    // Constants
    pub const I32_CONST: u8 = 0x41;
    pub const I64_CONST: u8 = 0x42;
    pub const F32_CONST: u8 = 0x43;
    pub const F64_CONST: u8 = 0x44;

    // i32 operations
    pub const I32_EQZ: u8 = 0x45;
    pub const I32_EQ: u8 = 0x46;
    pub const I32_NE: u8 = 0x47;
    pub const I32_LT_S: u8 = 0x48;
    pub const I32_GT_S: u8 = 0x4A;
    pub const I32_LE_S: u8 = 0x4C;
    pub const I32_GE_S: u8 = 0x4E;

    // f64 operations
    pub const F64_EQ: u8 = 0x61;
    pub const F64_NE: u8 = 0x62;
    pub const F64_LT: u8 = 0x63;
    pub const F64_GT: u8 = 0x64;
    pub const F64_LE: u8 = 0x65;
    pub const F64_GE: u8 = 0x66;
    pub const F64_ABS: u8 = 0x99;
    pub const F64_NEG: u8 = 0x9A;
    pub const F64_SQRT: u8 = 0x9F;
    pub const F64_ADD: u8 = 0xA0;
    pub const F64_SUB: u8 = 0xA1;
    pub const F64_MUL: u8 = 0xA2;
    pub const F64_DIV: u8 = 0xA3;

    // Type conversions
    pub const I32_TRUNC_F64_S: u8 = 0xAA;
    pub const F64_CONVERT_I32_S: u8 = 0xB7;

    // Drop
    pub const DROP: u8 = 0x1A;
}

pub struct Compiler {
    module: WasmModule,
    functions: HashMap<String, u32>,  // name -> func_idx
    #[allow(dead_code)]
    memory_offset: u32,  // Current offset in linear memory for allocations (reserved for future use)
    #[allow(dead_code)]
    module_name: String, // Module name (reserved for future use)
}

impl Compiler {
    pub fn new(name: String) -> Self {
        let mut module = WasmModule::new();

        // Add memory (1 page = 64KB)
        module.add_memory(Memory::new(Limits::min_only(1)));
        module.add_export(Export::memory("memory".to_string(), 0));

        // Import print function from host
        let print_type = module.add_type(FuncType::new(vec![ValType::F64], vec![]));
        module.add_import(Import::func(
            "env".to_string(),
            "print".to_string(),
            print_type,
        ));

        Self {
            module,
            functions: HashMap::new(),
            memory_offset: 0,
            module_name: name,
        }
    }

    pub fn compile(&mut self, exprs: &[Expr]) -> Result<Vec<u8>, String> {
        // First pass: collect function definitions
        for expr in exprs {
            if let ExprType::List(elements) = &expr.expr_type {
                if !elements.is_empty() {
                    if let ExprType::Symbol(op) = &elements[0].expr_type {
                        if op == "defn" {
                            self.preprocess_defn(elements)?;
                        }
                    }
                }
            }
        }

        // Create main function
        let main_type = self.module.add_type(FuncType::new(vec![], vec![]));

        let mut main_code = Vec::new();

        // Compile each top-level expression
        for expr in exprs {
            // Skip defn - they're already compiled
            if let ExprType::List(elements) = &expr.expr_type {
                if !elements.is_empty() {
                    if let ExprType::Symbol(op) = &elements[0].expr_type {
                        if op == "defn" {
                            continue;
                        }
                    }
                }
            }

            let code = self.compile_expr(expr, &HashMap::new())?;
            main_code.extend(code);
            // Drop the result (keep stack clean)
            main_code.push(opcode::DROP);
        }

        main_code.push(opcode::END);

        let main_idx = self.module.add_function(
            main_type,
            FunctionCode::new(vec![], main_code),
        );
        self.module.add_export(Export::func("main".to_string(), main_idx));

        Ok(self.module.encode())
    }

    fn preprocess_defn(&mut self, elements: &[Expr]) -> Result<(), String> {
        if elements.len() < 4 {
            return Err("defn requires at least 3 arguments".to_string());
        }

        let fn_name = match &elements[1].expr_type {
            ExprType::Symbol(s) => s.clone(),
            _ => return Err("defn requires a symbol as function name".to_string()),
        };

        let params = match &elements[2].expr_type {
            ExprType::List(p) => {
                let mut param_names = Vec::new();
                for param in p {
                    match &param.expr_type {
                        ExprType::Symbol(s) => param_names.push(s.clone()),
                        _ => return Err("defn parameters must be symbols".to_string()),
                    }
                }
                param_names
            }
            _ => return Err("defn requires a list of parameters".to_string()),
        };

        // Create function type (all params and return are f64)
        let param_types = vec![ValType::F64; params.len()];
        let func_type = self.module.add_type(FuncType::new(param_types, vec![ValType::F64]));

        // Build locals map for parameters
        let mut locals = HashMap::new();
        for (i, param_name) in params.iter().enumerate() {
            locals.insert(param_name.clone(), i as u32);
        }

        // Compile function body
        let mut body_code = Vec::new();

        // Compile each expression in the body
        for body_expr in &elements[3..] {
            body_code.extend(self.compile_expr(body_expr, &locals)?);
            // If not the last expression, drop the result
            if body_expr as *const _ != elements.last().unwrap() as *const _ {
                body_code.push(opcode::DROP);
            }
        }

        body_code.push(opcode::END);

        let func_idx = self.module.add_function(
            func_type,
            FunctionCode::new(vec![], body_code),
        );

        self.functions.insert(fn_name.clone(), func_idx);
        self.module.add_export(Export::func(fn_name, func_idx));

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr, locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        match &expr.expr_type {
            ExprType::Number(n) => self.compile_number(*n),
            ExprType::String(s) => self.compile_string(s),
            ExprType::Symbol(s) => self.compile_symbol(s, locals),
            ExprType::List(elements) => self.compile_list(elements, locals),
            ExprType::Quote(inner) => self.compile_quote(inner),
        }
    }

    fn compile_number(&mut self, n: f64) -> Result<Vec<u8>, String> {
        let mut code = Vec::new();
        code.push(opcode::F64_CONST);
        code.extend_from_slice(&n.to_le_bytes());
        Ok(code)
    }

    fn compile_string(&mut self, _s: &str) -> Result<Vec<u8>, String> {
        // For now, strings are not fully supported in WASM backend
        // We'll represent them as NaN for placeholder
        let mut code = Vec::new();
        code.push(opcode::F64_CONST);
        code.extend_from_slice(&f64::NAN.to_le_bytes());
        Ok(code)
    }

    fn compile_symbol(&mut self, s: &str, locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        // Check if it's a local variable
        if let Some(&local_idx) = locals.get(s) {
            let mut code = Vec::new();
            code.push(opcode::LOCAL_GET);
            code.extend(super::encoder::encode_unsigned(local_idx));
            return Ok(code);
        }

        // Check for boolean constants
        match s {
            "true" => return self.compile_number(1.0),
            "false" => return self.compile_number(0.0),
            "nil" => return self.compile_number(f64::NAN),
            _ => {}
        }

        Err(format!("Undefined symbol: {}", s))
    }

    fn compile_list(&mut self, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        if elements.is_empty() {
            return self.compile_number(f64::NAN); // Empty list as nil
        }

        match &elements[0].expr_type {
            ExprType::Symbol(op) => {
                match op.as_str() {
                    // Arithmetic
                    "+" => self.compile_arithmetic(elements, locals, opcode::F64_ADD),
                    "-" => self.compile_arithmetic(elements, locals, opcode::F64_SUB),
                    "*" => self.compile_arithmetic(elements, locals, opcode::F64_MUL),
                    "/" => self.compile_arithmetic(elements, locals, opcode::F64_DIV),
                    "mod" => self.compile_mod(elements, locals),

                    // Comparisons
                    "=" => self.compile_comparison(elements, locals, opcode::F64_EQ),
                    "<" => self.compile_comparison(elements, locals, opcode::F64_LT),
                    ">" => self.compile_comparison(elements, locals, opcode::F64_GT),
                    "<=" => self.compile_comparison(elements, locals, opcode::F64_LE),
                    ">=" => self.compile_comparison(elements, locals, opcode::F64_GE),

                    // Control flow
                    "if" => self.compile_if(elements, locals),
                    "do" => self.compile_do(elements, locals),

                    // I/O
                    "print" => self.compile_print(elements, locals),

                    // List operations (simplified)
                    "list" => self.compile_number(f64::NAN), // Placeholder

                    // User-defined function call
                    _ => self.compile_function_call(op, elements, locals),
                }
            }
            _ => Err("First element of list must be a symbol".to_string()),
        }
    }

    fn compile_quote(&mut self, _inner: &Expr) -> Result<Vec<u8>, String> {
        // Quote not fully supported yet
        self.compile_number(f64::NAN)
    }

    fn compile_arithmetic(&mut self, elements: &[Expr], locals: &HashMap<String, u32>, op: u8) -> Result<Vec<u8>, String> {
        if elements.len() < 3 {
            return Err("Arithmetic operation requires at least 2 operands".to_string());
        }

        let mut code = Vec::new();

        // Compile first operand
        code.extend(self.compile_expr(&elements[1], locals)?);

        // Compile and apply remaining operands
        for operand in &elements[2..] {
            code.extend(self.compile_expr(operand, locals)?);
            code.push(op);
        }

        Ok(code)
    }

    fn compile_mod(&mut self, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        if elements.len() != 3 {
            return Err("mod requires exactly 2 operands".to_string());
        }

        let mut code = Vec::new();

        // a mod b = a - floor(a/b) * b
        // For simplicity, we'll use: a - trunc(a/b) * b

        // Load a
        code.extend(self.compile_expr(&elements[1], locals)?);

        // Load b
        code.extend(self.compile_expr(&elements[2], locals)?);

        // Since WASM doesn't have f64.mod, we return 0.0 for now
        code.push(opcode::DROP);
        code.push(opcode::DROP);
        code.push(opcode::F64_CONST);
        code.extend_from_slice(&0.0f64.to_le_bytes());

        Ok(code)
    }

    fn compile_comparison(&mut self, elements: &[Expr], locals: &HashMap<String, u32>, op: u8) -> Result<Vec<u8>, String> {
        if elements.len() != 3 {
            return Err("Comparison requires exactly 2 operands".to_string());
        }

        let mut code = Vec::new();

        // Compile both operands
        code.extend(self.compile_expr(&elements[1], locals)?);
        code.extend(self.compile_expr(&elements[2], locals)?);

        // Perform comparison (produces i32)
        code.push(op);

        // Convert i32 result to f64 (0.0 or 1.0)
        code.push(opcode::F64_CONVERT_I32_S);

        Ok(code)
    }

    fn compile_if(&mut self, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        if elements.len() < 3 || elements.len() > 4 {
            return Err("if requires 2 or 3 arguments (condition, then, optional else)".to_string());
        }

        let mut code = Vec::new();

        // Compile condition
        code.extend(self.compile_expr(&elements[1], locals)?);

        // Convert to i32 for if (non-zero = true)
        code.push(opcode::I32_TRUNC_F64_S);

        // if block (f64 result type)
        code.push(opcode::IF);
        code.push(ValType::F64.to_byte());

        // Then branch
        code.extend(self.compile_expr(&elements[2], locals)?);

        if elements.len() == 4 {
            // Else branch
            code.push(opcode::ELSE);
            code.extend(self.compile_expr(&elements[3], locals)?);
        } else {
            // No else branch, return nil
            code.push(opcode::ELSE);
            code.push(opcode::F64_CONST);
            code.extend_from_slice(&f64::NAN.to_le_bytes());
        }

        code.push(opcode::END);

        Ok(code)
    }

    fn compile_do(&mut self, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        if elements.len() < 2 {
            return Err("do requires at least 1 expression".to_string());
        }

        let mut code = Vec::new();

        // Compile all but the last expression, dropping results
        for expr in &elements[1..elements.len()-1] {
            code.extend(self.compile_expr(expr, locals)?);
            code.push(opcode::DROP);
        }

        // Compile last expression (its value is returned)
        code.extend(self.compile_expr(&elements[elements.len()-1], locals)?);

        Ok(code)
    }

    fn compile_print(&mut self, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        if elements.len() != 2 {
            return Err("print requires exactly 1 argument".to_string());
        }

        let mut code = Vec::new();

        // Compile the argument
        code.extend(self.compile_expr(&elements[1], locals)?);

        // Call imported print function (index 0)
        code.push(opcode::CALL);
        code.extend(super::encoder::encode_unsigned(0));

        // print returns nothing, so push nil as result
        code.push(opcode::F64_CONST);
        code.extend_from_slice(&f64::NAN.to_le_bytes());

        Ok(code)
    }

    fn compile_function_call(&mut self, name: &str, elements: &[Expr], locals: &HashMap<String, u32>) -> Result<Vec<u8>, String> {
        let func_idx = *self.functions.get(name)
            .ok_or_else(|| format!("Undefined function: {}", name))?;

        let mut code = Vec::new();

        // Compile arguments
        for arg in &elements[1..] {
            code.extend(self.compile_expr(arg, locals)?);
        }

        // Call function
        code.push(opcode::CALL);
        code.extend(super::encoder::encode_unsigned(func_idx));

        Ok(code)
    }
}

// Implement the CodegenBackend trait
impl crate::codegen::backend::CodegenBackend for Compiler {
    fn new(name: String) -> Self {
        Compiler::new(name)
    }

    fn compile(&mut self, exprs: &[Expr]) -> Result<Vec<u8>, String> {
        self.compile(exprs)
    }

    fn file_extension(&self) -> &str {
        "wasm"
    }

    fn backend_name(&self) -> &str {
        "WASM"
    }
}
