/// REPLisp to JVM Bytecode Compiler

use crate::parser::{Expr, ExprType};
use super::bytecode::Opcode;
use super::classfile::{ClassFile, MethodInfo, FieldInfo, AccessFlags, ConstantPool};
use std::collections::HashMap;

pub struct Compiler {
    class: ClassFile,
    globals: HashMap<String, u16>, // Global variable -> field index
    local_counter: usize,
}

impl Compiler {
    pub fn new(class_name: String) -> Self {
        let mut class = ClassFile::new(class_name);

        Compiler {
            class,
            globals: HashMap::new(),
            local_counter: 0,
        }
    }

    /// Compile a list of expressions to a class file
    pub fn compile(&mut self, exprs: &[Expr]) -> Result<Vec<u8>, String> {
        // Create main method
        let mut main_method = MethodInfo::new(
            AccessFlags::public_static(),
            "main".to_string(),
            "([Ljava/lang/String;)V".to_string(),
        );

        // Compile each expression and add to main method
        for expr in exprs {
            let bytecode = self.compile_expr(expr, &mut HashMap::new())?;
            main_method.code.extend(bytecode);

            // Pop the result if not used (keep stack clean)
            main_method.code.extend(Opcode::Pop.encode());
        }

        // Return
        main_method.code.extend(Opcode::Return.encode());

        self.class.add_method(main_method);

        Ok(self.class.write())
    }

    /// Compile a single expression
    fn compile_expr(&mut self, expr: &Expr, locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        match &expr.expr_type {
            ExprType::Number(n) => self.compile_number(*n),
            ExprType::String(s) => self.compile_string(s),
            ExprType::Symbol(s) => self.compile_symbol(s, locals),
            ExprType::List(elements) => self.compile_list(elements, locals),
            ExprType::Quote(inner) => self.compile_quote(inner),
        }
    }

    /// Compile a number literal
    fn compile_number(&mut self, n: f64) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Create a Double object (boxed)
        // Get constant pool index for the double value
        let double_index = self.class.constant_pool.add_double(n);

        // Load the double constant
        bytecode.extend(Opcode::Ldc2_w(double_index).encode());

        // Box it: Double.valueOf(double)
        let valueof_ref = self.class.constant_pool.add_methodref(
            "java/lang/Double".to_string(),
            "valueOf".to_string(),
            "(D)Ljava/lang/Double;".to_string(),
        );
        bytecode.extend(Opcode::Invokestatic(valueof_ref).encode());

        Ok(bytecode)
    }

    /// Compile a string literal
    fn compile_string(&mut self, s: &str) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        let string_index = self.class.constant_pool.add_string(s.to_string());
        bytecode.extend(Opcode::Ldc_w(string_index).encode());

        Ok(bytecode)
    }

    /// Compile a symbol (variable reference)
    fn compile_symbol(&mut self, s: &str, locals: &HashMap<String, u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Check if it's a local variable
        if let Some(&local_idx) = locals.get(s) {
            bytecode.extend(Opcode::Aload(local_idx).encode());
            return Ok(bytecode);
        }

        // Check if it's a global variable (field)
        if let Some(&field_idx) = self.globals.get(s) {
            // Load from static field
            bytecode.extend(Opcode::Getstatic(field_idx).encode());
            return Ok(bytecode);
        }

        // Handle boolean literals
        match s {
            "true" => {
                let true_ref = self.class.constant_pool.add_fieldref(
                    "java/lang/Boolean".to_string(),
                    "TRUE".to_string(),
                    "Ljava/lang/Boolean;".to_string(),
                );
                bytecode.extend(Opcode::Getstatic(true_ref).encode());
                return Ok(bytecode);
            }
            "false" => {
                let false_ref = self.class.constant_pool.add_fieldref(
                    "java/lang/Boolean".to_string(),
                    "FALSE".to_string(),
                    "Ljava/lang/Boolean;".to_string(),
                );
                bytecode.extend(Opcode::Getstatic(false_ref).encode());
                return Ok(bytecode);
            }
            "nil" => {
                bytecode.extend(Opcode::Aconst_null.encode());
                return Ok(bytecode);
            }
            _ => {}
        }

        Err(format!("Undefined variable: {}", s))
    }

    /// Compile a list (function call or special form)
    fn compile_list(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        if elements.is_empty() {
            return Ok(Opcode::Aconst_null.encode());
        }

        // Check if first element is a special form
        if let ExprType::Symbol(op) = &elements[0].expr_type {
            match op.as_str() {
                "def" => return self.compile_def(elements, locals),
                "defn" => return self.compile_defn(elements, locals),
                "if" => return self.compile_if(elements, locals),
                "do" => return self.compile_do(elements, locals),
                "print" => return self.compile_print(elements, locals),
                "+" | "-" | "*" | "/" | "=" | "<" | ">" | "<=" | ">=" => {
                    return self.compile_arithmetic(op, &elements[1..], locals);
                }
                _ => {}
            }
        }

        // Regular function call - for now, error
        Err("Function calls not yet implemented".to_string())
    }

    /// Compile def special form
    fn compile_def(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        if elements.len() != 3 {
            return Err("def requires exactly 2 arguments".to_string());
        }

        let var_name = match &elements[1].expr_type {
            ExprType::Symbol(s) => s.clone(),
            _ => return Err("def requires a symbol as first argument".to_string()),
        };

        let mut bytecode = Vec::new();

        // Compile the value
        bytecode.extend(self.compile_expr(&elements[2], locals)?);

        // Store in a static field
        let field = FieldInfo::new(
            AccessFlags::public_static(),
            var_name.clone(),
            "Ljava/lang/Object;".to_string(),
        );
        self.class.add_field(field);

        let field_ref = self.class.constant_pool.add_fieldref(
            self.class.this_class.clone(),
            var_name.clone(),
            "Ljava/lang/Object;".to_string(),
        );
        self.globals.insert(var_name, field_ref);

        // Duplicate the value (one for storing, one for returning)
        bytecode.extend(Opcode::Dup.encode());
        bytecode.extend(Opcode::Putstatic(field_ref).encode());

        Ok(bytecode)
    }

    /// Compile defn special form (simplified - creates a static method)
    fn compile_defn(&mut self, _elements: &[Expr], _locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        // For now, defn is not fully implemented in bytecode compilation
        Err("defn not yet fully implemented in bytecode compiler".to_string())
    }

    /// Compile if special form
    fn compile_if(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        if elements.len() < 3 || elements.len() > 4 {
            return Err("if requires 2 or 3 arguments".to_string());
        }

        let mut bytecode = Vec::new();

        // Compile condition
        bytecode.extend(self.compile_expr(&elements[1], locals)?);

        // Check if condition is true (assuming it's a Boolean object)
        // Call Boolean.booleanValue()
        let booleanvalue_ref = self.class.constant_pool.add_methodref(
            "java/lang/Boolean".to_string(),
            "booleanValue".to_string(),
            "()Z".to_string(),
        );
        bytecode.extend(Opcode::Checkcast(
            self.class.constant_pool.add_class("java/lang/Boolean".to_string())
        ).encode());
        bytecode.extend(Opcode::Invokevirtual(booleanvalue_ref).encode());

        // If false, jump to else branch
        let else_label_offset = bytecode.len();
        bytecode.extend(Opcode::Ifeq(0).encode()); // Placeholder offset

        // Then branch
        let then_start = bytecode.len();
        bytecode.extend(self.compile_expr(&elements[2], locals)?);

        // Jump to end
        let end_jump_offset = bytecode.len();
        bytecode.extend(Opcode::Goto(0).encode()); // Placeholder offset

        // Else branch
        let else_start = bytecode.len();
        if elements.len() == 4 {
            bytecode.extend(self.compile_expr(&elements[3], locals)?);
        } else {
            bytecode.extend(Opcode::Aconst_null.encode());
        }

        let end_label = bytecode.len();

        // Fix up jump offsets
        let else_offset = (else_start as i16) - (else_label_offset as i16);
        let end_offset = (end_label as i16) - (end_jump_offset as i16);

        bytecode[else_label_offset + 1] = (else_offset >> 8) as u8;
        bytecode[else_label_offset + 2] = else_offset as u8;

        bytecode[end_jump_offset + 1] = (end_offset >> 8) as u8;
        bytecode[end_jump_offset + 2] = end_offset as u8;

        Ok(bytecode)
    }

    /// Compile do special form
    fn compile_do(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        for (i, expr) in elements[1..].iter().enumerate() {
            bytecode.extend(self.compile_expr(expr, locals)?);

            // Pop all but the last result
            if i < elements.len() - 2 {
                bytecode.extend(Opcode::Pop.encode());
            }
        }

        // If no expressions, return nil
        if elements.len() == 1 {
            bytecode.extend(Opcode::Aconst_null.encode());
        }

        Ok(bytecode)
    }

    /// Compile print function
    fn compile_print(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Get System.out
        let out_ref = self.class.constant_pool.add_fieldref(
            "java/lang/System".to_string(),
            "out".to_string(),
            "Ljava/io/PrintStream;".to_string(),
        );

        let println_ref = self.class.constant_pool.add_methodref(
            "java/io/PrintStream".to_string(),
            "println".to_string(),
            "(Ljava/lang/Object;)V".to_string(),
        );

        // Compile and print each argument
        for expr in &elements[1..] {
            bytecode.extend(Opcode::Getstatic(out_ref).encode()); // Get System.out
            bytecode.extend(self.compile_expr(expr, locals)?);     // Compile argument
            bytecode.extend(Opcode::Invokevirtual(println_ref).encode()); // Call println
        }

        // Return nil
        bytecode.extend(Opcode::Aconst_null.encode());

        Ok(bytecode)
    }

    /// Compile arithmetic operations
    fn compile_arithmetic(&mut self, op: &str, args: &[Expr], locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
        if args.is_empty() {
            return Err(format!("{} requires at least one argument", op));
        }

        let mut bytecode = Vec::new();

        // Compile first argument
        bytecode.extend(self.compile_expr(&args[0], locals)?);
        bytecode.extend(self.unbox_double()?);

        // Process remaining arguments
        for arg in &args[1..] {
            bytecode.extend(self.compile_expr(arg, locals)?);
            bytecode.extend(self.unbox_double()?);

            match op {
                "+" => bytecode.extend(Opcode::Dadd.encode()),
                "-" => bytecode.extend(Opcode::Dsub.encode()),
                "*" => bytecode.extend(Opcode::Dmul.encode()),
                "/" => bytecode.extend(Opcode::Ddiv.encode()),
                _ => {}
            }
        }

        // Handle comparison operators
        if matches!(op, "=" | "<" | ">" | "<=" | ">=") {
            bytecode.extend(Opcode::Dcmpl.encode());

            // Pattern: if_cond -> jump to true, false path, goto end, true path
            // Getstatic is 3 bytes, Goto is 3 bytes
            // So: jump over (3 + 3) = 6 bytes to reach true path
            // And goto needs to jump over 3 bytes (the true load) to end

            match op {
                "=" => {
                    // If equal (result == 0)
                    bytecode.extend(Opcode::Ifeq(6).encode()); // Jump 6 bytes if equal
                    bytecode.extend(self.load_boolean(false)?); // 3 bytes
                    bytecode.extend(Opcode::Goto(3).encode());  // Jump 3 bytes
                    bytecode.extend(self.load_boolean(true)?);  // 3 bytes
                }
                "<" => {
                    // If less than (result < 0)
                    bytecode.extend(Opcode::Iflt(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                ">" => {
                    // If greater than (result > 0)
                    bytecode.extend(Opcode::Ifgt(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                "<=" => {
                    // If less than or equal (result <= 0)
                    bytecode.extend(Opcode::Ifle(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                ">=" => {
                    // If greater than or equal (result >= 0)
                    bytecode.extend(Opcode::Ifge(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                _ => {}
            }
            return Ok(bytecode);
        }

        // Box the result for non-comparison operations
        bytecode.extend(self.box_double()?);

        Ok(bytecode)
    }

    /// Compile quote special form
    fn compile_quote(&mut self, _inner: &Expr) -> Result<Vec<u8>, String> {
        // For now, return nil for quoted expressions
        Ok(Opcode::Aconst_null.encode())
    }

    /// Helper: Unbox a Double to a primitive double
    fn unbox_double(&mut self) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Cast to Double
        let double_class = self.class.constant_pool.add_class("java/lang/Double".to_string());
        bytecode.extend(Opcode::Checkcast(double_class).encode());

        // Call doubleValue()
        let doublevalue_ref = self.class.constant_pool.add_methodref(
            "java/lang/Double".to_string(),
            "doubleValue".to_string(),
            "()D".to_string(),
        );
        bytecode.extend(Opcode::Invokevirtual(doublevalue_ref).encode());

        Ok(bytecode)
    }

    /// Helper: Box a primitive double to Double
    fn box_double(&mut self) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        let valueof_ref = self.class.constant_pool.add_methodref(
            "java/lang/Double".to_string(),
            "valueOf".to_string(),
            "(D)Ljava/lang/Double;".to_string(),
        );
        bytecode.extend(Opcode::Invokestatic(valueof_ref).encode());

        Ok(bytecode)
    }

    /// Helper: Load a Boolean constant
    fn load_boolean(&mut self, value: bool) -> Result<Vec<u8>, String> {
        let field_name = if value { "TRUE" } else { "FALSE" };
        let field_ref = self.class.constant_pool.add_fieldref(
            "java/lang/Boolean".to_string(),
            field_name.to_string(),
            "Ljava/lang/Boolean;".to_string(),
        );
        Ok(Opcode::Getstatic(field_ref).encode())
    }
}
