/// REPLisp to JVM Bytecode Compiler with Full Language Support

use crate::parser::{Expr, ExprType};
use super::bytecode::Opcode;
use super::classfile::{ClassFile, MethodInfo, FieldInfo, AccessFlags};
use std::collections::HashMap;

pub struct Compiler {
    class: ClassFile,
    globals: HashMap<String, GlobalBinding>,
    functions: HashMap<String, FunctionInfo>,
    lambda_counter: usize,
}

#[derive(Clone)]
enum GlobalBinding {
    Variable(u16), // Field ref index
    Function(String), // Method name
}

struct FunctionInfo {
    method_name: String,
    param_count: usize,
}

impl Compiler {
    pub fn new(class_name: String) -> Self {
        Compiler {
            class: ClassFile::new(class_name),
            globals: HashMap::new(),
            functions: HashMap::new(),
            lambda_counter: 0,
        }
    }

    /// Compile a list of expressions to a class file
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

        // Create main method
        let mut main_method = MethodInfo::new(
            AccessFlags::public_static(),
            "main".to_string(),
            "([Ljava/lang/String;)V".to_string(),
        );

        main_method.max_stack = 100;
        main_method.max_locals = 10;

        // Compile each expression and add to main method
        for expr in exprs {
            let bytecode = self.compile_expr(expr, &mut HashMap::new(), &mut main_method.code)?;
            main_method.code.extend(bytecode);

            // Pop the result if not used (keep stack clean)
            main_method.code.extend(Opcode::Pop.encode());
        }

        // Return
        main_method.code.extend(Opcode::Return.encode());

        self.class.add_method(main_method);

        Ok(self.class.write())
    }

    /// Preprocess defn to register function
    fn preprocess_defn(&mut self, elements: &[Expr]) -> Result<(), String> {
        if elements.len() < 4 {
            return Err("defn requires at least 3 arguments".to_string());
        }

        let fn_name = match &elements[1].expr_type {
            ExprType::Symbol(s) => s.clone(),
            _ => return Err("defn requires a symbol as function name".to_string()),
        };

        let params = match &elements[2].expr_type {
            ExprType::List(p) => p,
            _ => return Err("defn requires a list of parameters".to_string()),
        };

        let method_name = format!("fn_{}", fn_name);

        self.functions.insert(fn_name.clone(), FunctionInfo {
            method_name: method_name.clone(),
            param_count: params.len(),
        });

        self.globals.insert(fn_name, GlobalBinding::Function(method_name));

        Ok(())
    }

    /// Compile a single expression
    fn compile_expr(&mut self, expr: &Expr, locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        match &expr.expr_type {
            ExprType::Number(n) => self.compile_number(*n),
            ExprType::String(s) => self.compile_string(s),
            ExprType::Symbol(s) => self.compile_symbol(s, locals),
            ExprType::List(elements) => self.compile_list(elements, locals, code_buffer),
            ExprType::Quote(inner) => self.compile_quote(inner),
        }
    }

    /// Compile a number literal
    fn compile_number(&mut self, n: f64) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Load the double constant
        let double_index = self.class.constant_pool.add_double(n);
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

        // Check if it's a global
        if let Some(binding) = self.globals.get(s).cloned() {
            match binding {
                GlobalBinding::Variable(field_idx) => {
                    bytecode.extend(Opcode::Getstatic(field_idx).encode());
                    return Ok(bytecode);
                }
                GlobalBinding::Function(_method_name) => {
                    // Return a method reference wrapper (simplified: return null for now)
                    // In a full implementation, we'd create a Function object
                    bytecode.extend(Opcode::Aconst_null.encode());
                    return Ok(bytecode);
                }
            }
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
    fn compile_list(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        if elements.is_empty() {
            // Empty list
            return self.compile_empty_list();
        }

        // Check if first element is a special form
        if let ExprType::Symbol(op) = &elements[0].expr_type {
            match op.as_str() {
                "def" => return self.compile_def(elements, locals, code_buffer),
                "defn" => return self.compile_defn(elements, locals),
                "lambda" => return self.compile_lambda(elements, locals, code_buffer),
                "if" => return self.compile_if(elements, locals, code_buffer),
                "do" => return self.compile_do(elements, locals, code_buffer),
                "print" => return self.compile_print(elements, locals, code_buffer),
                "list" => return self.compile_list_constructor(elements, locals, code_buffer),
                "+" | "-" | "*" | "/" | "=" | "<" | ">" | "<=" | ">=" | "mod" => {
                    return self.compile_arithmetic(op, &elements[1..], locals, code_buffer);
                }
                "car" | "cdr" | "cons" | "length" | "null?" | "reverse" => {
                    return self.compile_list_operation(op, &elements[1..], locals, code_buffer);
                }
                _ => {
                    // Check if it's a known function
                    if self.functions.contains_key(op) {
                        return self.compile_function_call(op, &elements[1..], locals, code_buffer);
                    }
                }
            }
        }

        // General function call
        self.compile_general_call(&elements[0], &elements[1..], locals, code_buffer)
    }

    /// Compile empty list
    fn compile_empty_list(&mut self) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // new ArrayList()
        let arraylist_class = self.class.constant_pool.add_class("java/util/ArrayList".to_string());
        bytecode.extend(Opcode::New(arraylist_class).encode());
        bytecode.extend(Opcode::Dup.encode());

        let init_ref = self.class.constant_pool.add_methodref(
            "java/util/ArrayList".to_string(),
            "<init>".to_string(),
            "()V".to_string(),
        );
        bytecode.extend(Opcode::Invokespecial(init_ref).encode());

        Ok(bytecode)
    }

    /// Compile list constructor
    fn compile_list_constructor(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Create new ArrayList
        let arraylist_class = self.class.constant_pool.add_class("java/util/ArrayList".to_string());
        bytecode.extend(Opcode::New(arraylist_class).encode());
        bytecode.extend(Opcode::Dup.encode());

        let init_ref = self.class.constant_pool.add_methodref(
            "java/util/ArrayList".to_string(),
            "<init>".to_string(),
            "()V".to_string(),
        );
        bytecode.extend(Opcode::Invokespecial(init_ref).encode());

        let add_ref = self.class.constant_pool.add_methodref(
            "java/util/ArrayList".to_string(),
            "add".to_string(),
            "(Ljava/lang/Object;)Z".to_string(),
        );

        // Add each element
        for elem in &elements[1..] {
            bytecode.extend(Opcode::Dup.encode()); // Dup the list reference
            bytecode.extend(self.compile_expr(elem, locals, code_buffer)?);
            bytecode.extend(Opcode::Invokevirtual(add_ref).encode());
            bytecode.extend(Opcode::Pop.encode()); // Pop the boolean return value
        }

        Ok(bytecode)
    }

    /// Compile list operations
    fn compile_list_operation(&mut self, op: &str, args: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        match op {
            "car" => {
                if args.len() != 1 {
                    return Err("car requires exactly 1 argument".to_string());
                }
                // Get first element: list.get(0)
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/List".to_string())
                ).encode());
                bytecode.extend(Opcode::Iconst_0.encode());
                let get_ref = self.class.constant_pool.add_interface_methodref(
                    "java/util/List".to_string(),
                    "get".to_string(),
                    "(I)Ljava/lang/Object;".to_string(),
                );
                bytecode.extend(Opcode::Invokeinterface(get_ref, 2).encode());
            }
            "cdr" => {
                if args.len() != 1 {
                    return Err("cdr requires exactly 1 argument".to_string());
                }
                // Return new ArrayList with sublist from index 1
                // Store list in local variable first to avoid stack issues
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/List".to_string())
                ).encode());

                // Create new ArrayList
                bytecode.extend(self.compile_empty_list()?);
                bytecode.extend(Opcode::Swap.encode());  // Now: newlist, oldlist
                bytecode.extend(Opcode::Dup.encode());   // Now: newlist, oldlist, oldlist
                bytecode.extend(Opcode::Iconst_1.encode());  // Now: newlist, oldlist, oldlist, 1
                bytecode.extend(Opcode::Swap.encode());  // Now: newlist, oldlist, 1, oldlist
                bytecode.extend(Opcode::Dup.encode());   // Now: newlist, oldlist, 1, oldlist, oldlist

                let size_ref = self.class.constant_pool.add_interface_methodref(
                    "java/util/List".to_string(),
                    "size".to_string(),
                    "()I".to_string(),
                );
                bytecode.extend(Opcode::Invokeinterface(size_ref, 1).encode());
                // Now: newlist, oldlist, 1, oldlist, size

                let sublist_ref = self.class.constant_pool.add_interface_methodref(
                    "java/util/List".to_string(),
                    "subList".to_string(),
                    "(II)Ljava/util/List;".to_string(),
                );
                bytecode.extend(Opcode::Invokeinterface(sublist_ref, 3).encode());
                // Now: newlist, sublist

                // Add sublist to new list
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/Collection".to_string())
                ).encode());

                let addall_ref = self.class.constant_pool.add_methodref(
                    "java/util/ArrayList".to_string(),
                    "addAll".to_string(),
                    "(Ljava/util/Collection;)Z".to_string(),
                );
                bytecode.extend(Opcode::Swap.encode());  // newlist, sublist -> sublist, newlist
                bytecode.extend(Opcode::Dup_x1.encode()); // sublist, newlist -> newlist, sublist, newlist
                bytecode.extend(Opcode::Swap.encode());  // newlist, sublist, newlist -> newlist, newlist, sublist
                bytecode.extend(Opcode::Invokevirtual(addall_ref).encode());
                bytecode.extend(Opcode::Pop.encode());   // Pop boolean result
            }
            "cons" => {
                if args.len() != 2 {
                    return Err("cons requires exactly 2 arguments".to_string());
                }
                // Create new list with element at front
                bytecode.extend(self.compile_empty_list()?);
                bytecode.extend(Opcode::Dup.encode());
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);

                let add_ref = self.class.constant_pool.add_methodref(
                    "java/util/ArrayList".to_string(),
                    "add".to_string(),
                    "(Ljava/lang/Object;)Z".to_string(),
                );
                bytecode.extend(Opcode::Invokevirtual(add_ref).encode());
                bytecode.extend(Opcode::Pop.encode());

                // Add all elements from second list
                bytecode.extend(Opcode::Dup.encode());
                bytecode.extend(self.compile_expr(&args[1], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/Collection".to_string())
                ).encode());

                let addall_ref = self.class.constant_pool.add_methodref(
                    "java/util/ArrayList".to_string(),
                    "addAll".to_string(),
                    "(Ljava/util/Collection;)Z".to_string(),
                );
                bytecode.extend(Opcode::Invokevirtual(addall_ref).encode());
                bytecode.extend(Opcode::Pop.encode());
            }
            "length" => {
                if args.len() != 1 {
                    return Err("length requires exactly 1 argument".to_string());
                }
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/List".to_string())
                ).encode());

                let size_ref = self.class.constant_pool.add_interface_methodref(
                    "java/util/List".to_string(),
                    "size".to_string(),
                    "()I".to_string(),
                );
                bytecode.extend(Opcode::Invokeinterface(size_ref, 1).encode());

                // Box to Double
                bytecode.extend(Opcode::I2d.encode());
                let valueof_ref = self.class.constant_pool.add_methodref(
                    "java/lang/Double".to_string(),
                    "valueOf".to_string(),
                    "(D)Ljava/lang/Double;".to_string(),
                );
                bytecode.extend(Opcode::Invokestatic(valueof_ref).encode());
            }
            "null?" => {
                if args.len() != 1 {
                    return Err("null? requires exactly 1 argument".to_string());
                }
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/List".to_string())
                ).encode());

                let isempty_ref = self.class.constant_pool.add_interface_methodref(
                    "java/util/List".to_string(),
                    "isEmpty".to_string(),
                    "()Z".to_string(),
                );
                bytecode.extend(Opcode::Invokeinterface(isempty_ref, 1).encode());

                // Convert to Boolean
                bytecode.extend(Opcode::Ifeq(6).encode());
                bytecode.extend(self.load_boolean(false)?);
                bytecode.extend(Opcode::Goto(3).encode());
                bytecode.extend(self.load_boolean(true)?);
            }
            "reverse" => {
                if args.len() != 1 {
                    return Err("reverse requires exactly 1 argument".to_string());
                }
                // Create new ArrayList, add all elements in reverse
                bytecode.extend(self.compile_empty_list()?);
                bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
                bytecode.extend(Opcode::Checkcast(
                    self.class.constant_pool.add_class("java/util/List".to_string())
                ).encode());

                let addall_ref = self.class.constant_pool.add_methodref(
                    "java/util/ArrayList".to_string(),
                    "addAll".to_string(),
                    "(Ljava/util/Collection;)Z".to_string(),
                );
                bytecode.extend(Opcode::Swap.encode());
                bytecode.extend(Opcode::Dup_x1.encode());
                bytecode.extend(Opcode::Swap.encode());
                bytecode.extend(Opcode::Invokevirtual(addall_ref).encode());
                bytecode.extend(Opcode::Pop.encode());

                let reverse_ref = self.class.constant_pool.add_methodref(
                    "java/util/Collections".to_string(),
                    "reverse".to_string(),
                    "(Ljava/util/List;)V".to_string(),
                );
                bytecode.extend(Opcode::Dup.encode());
                bytecode.extend(Opcode::Invokestatic(reverse_ref).encode());
            }
            _ => return Err(format!("Unknown list operation: {}", op)),
        }

        Ok(bytecode)
    }

    /// Compile def special form
    fn compile_def(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        if elements.len() != 3 {
            return Err("def requires exactly 2 arguments".to_string());
        }

        let var_name = match &elements[1].expr_type {
            ExprType::Symbol(s) => s.clone(),
            _ => return Err("def requires a symbol as first argument".to_string()),
        };

        let mut bytecode = Vec::new();

        // Compile the value
        bytecode.extend(self.compile_expr(&elements[2], locals, code_buffer)?);

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
        self.globals.insert(var_name, GlobalBinding::Variable(field_ref));

        // Duplicate the value (one for storing, one for returning)
        bytecode.extend(Opcode::Dup.encode());
        bytecode.extend(Opcode::Putstatic(field_ref).encode());

        Ok(bytecode)
    }

    /// Compile defn special form
    fn compile_defn(&mut self, elements: &[Expr], _locals: &mut HashMap<String, u8>) -> Result<Vec<u8>, String> {
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

        let body = &elements[3..];

        // Create method
        let method_name = format!("fn_{}", fn_name);
        let mut method = MethodInfo::new(
            AccessFlags::public_static(),
            method_name.clone(),
            self.create_method_descriptor(params.len()),
        );

        method.max_stack = 100;
        method.max_locals = (params.len() + 1) as u16;

        // Map parameters to local variables
        let mut fn_locals = HashMap::new();
        for (i, param) in params.iter().enumerate() {
            fn_locals.insert(param.clone(), i as u8);
        }

        // Compile body
        for (i, expr) in body.iter().enumerate() {
            let bytecode = self.compile_expr(expr, &mut fn_locals, &mut method.code)?;
            method.code.extend(bytecode);

            // Pop all but the last result
            if i < body.len() - 1 {
                method.code.extend(Opcode::Pop.encode());
            }
        }

        // Return the last value
        method.code.extend(Opcode::Areturn.encode());

        self.class.add_method(method);

        // Return nil (defn doesn't return a value in the main flow)
        Ok(Opcode::Aconst_null.encode())
    }

    /// Create method descriptor for function with n parameters
    fn create_method_descriptor(&self, param_count: usize) -> String {
        let params = "Ljava/lang/Object;".repeat(param_count);
        format!("({})Ljava/lang/Object;", params)
    }

    /// Compile lambda special form
    fn compile_lambda(&mut self, elements: &[Expr], _locals: &mut HashMap<String, u8>, _code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        if elements.len() < 3 {
            return Err("lambda requires at least 2 arguments".to_string());
        }

        let params = match &elements[1].expr_type {
            ExprType::List(p) => {
                let mut param_names = Vec::new();
                for param in p {
                    match &param.expr_type {
                        ExprType::Symbol(s) => param_names.push(s.clone()),
                        _ => return Err("lambda parameters must be symbols".to_string()),
                    }
                }
                param_names
            }
            _ => return Err("lambda requires a list of parameters".to_string()),
        };

        let body = &elements[2..];

        // Generate unique lambda name
        let lambda_name = format!("lambda_{}", self.lambda_counter);
        self.lambda_counter += 1;

        // Create static method for lambda
        let mut method = MethodInfo::new(
            AccessFlags::public_static(),
            lambda_name.clone(),
            self.create_method_descriptor(params.len()),
        );

        method.max_stack = 100;
        method.max_locals = (params.len() + 1) as u16;

        // Map parameters to local variables
        let mut fn_locals = HashMap::new();
        for (i, param) in params.iter().enumerate() {
            fn_locals.insert(param.clone(), i as u8);
        }

        // Compile body
        for (i, expr) in body.iter().enumerate() {
            let bytecode = self.compile_expr(expr, &mut fn_locals, &mut method.code)?;
            method.code.extend(bytecode);

            if i < body.len() - 1 {
                method.code.extend(Opcode::Pop.encode());
            }
        }

        method.code.extend(Opcode::Areturn.encode());

        self.class.add_method(method);

        // For now, return null (would need to create a Function wrapper object)
        Ok(Opcode::Aconst_null.encode())
    }

    /// Compile function call
    fn compile_function_call(&mut self, fn_name: &str, args: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Get function info
        let (method_name, param_count) = {
            let fn_info = self.functions.get(fn_name)
                .ok_or_else(|| format!("Unknown function: {}", fn_name))?;
            (fn_info.method_name.clone(), fn_info.param_count)
        };

        if args.len() != param_count {
            return Err(format!("{} expects {} arguments, got {}", fn_name, param_count, args.len()));
        }

        // Push arguments onto stack
        for arg in args {
            bytecode.extend(self.compile_expr(arg, locals, code_buffer)?);
        }

        // Call the method
        let method_ref = self.class.constant_pool.add_methodref(
            self.class.this_class.clone(),
            method_name,
            self.create_method_descriptor(param_count),
        );
        bytecode.extend(Opcode::Invokestatic(method_ref).encode());

        Ok(bytecode)
    }

    /// Compile general function call (for lambdas, etc.)
    fn compile_general_call(&mut self, _fn_expr: &Expr, _args: &[Expr], _locals: &mut HashMap<String, u8>, _code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        // For now, not fully implemented
        Err("General function calls (lambdas as values) not yet fully implemented".to_string())
    }

    /// Compile if special form
    fn compile_if(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        if elements.len() < 3 || elements.len() > 4 {
            return Err("if requires 2 or 3 arguments".to_string());
        }

        let mut bytecode = Vec::new();

        // Compile condition
        bytecode.extend(self.compile_expr(&elements[1], locals, code_buffer)?);

        // Check if condition is truthy
        bytecode.extend(self.is_truthy()?);

        // If false, jump to else branch
        let else_label_offset = bytecode.len();
        bytecode.extend(Opcode::Ifeq(0).encode()); // Placeholder offset

        // Then branch
        let then_bytecode = self.compile_expr(&elements[2], locals, code_buffer)?;
        bytecode.extend(&then_bytecode);

        // Jump to end
        let end_jump_offset = bytecode.len();
        bytecode.extend(Opcode::Goto(0).encode()); // Placeholder offset

        // Else branch
        let else_start = bytecode.len();
        if elements.len() == 4 {
            bytecode.extend(self.compile_expr(&elements[3], locals, code_buffer)?);
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

    /// Helper: Check if value is truthy (not null and not false)
    fn is_truthy(&mut self) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        // Dup the value
        bytecode.extend(Opcode::Dup.encode());

        // Check if null
        bytecode.extend(Opcode::Ifnull(6).encode()); // If null, return false

        // Try to cast to Boolean and check
        bytecode.extend(Opcode::Dup.encode());
        let boolean_class = self.class.constant_pool.add_class("java/lang/Boolean".to_string());
        bytecode.extend(Opcode::Instanceof(boolean_class).encode());
        bytecode.extend(Opcode::Ifeq(11).encode()); // If not Boolean, it's truthy

        // It's a Boolean, check its value
        bytecode.extend(Opcode::Checkcast(boolean_class).encode());
        let booleanvalue_ref = self.class.constant_pool.add_methodref(
            "java/lang/Boolean".to_string(),
            "booleanValue".to_string(),
            "()Z".to_string(),
        );
        bytecode.extend(Opcode::Invokevirtual(booleanvalue_ref).encode());
        bytecode.extend(Opcode::Goto(4).encode());

        // Not a Boolean, pop and return true
        bytecode.extend(Opcode::Pop.encode());
        bytecode.extend(Opcode::Iconst_1.encode());
        bytecode.extend(Opcode::Goto(1).encode());

        // Null case, return false
        bytecode.extend(Opcode::Pop.encode());
        bytecode.extend(Opcode::Iconst_0.encode());

        Ok(bytecode)
    }

    /// Compile do special form
    fn compile_do(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

        for (i, expr) in elements[1..].iter().enumerate() {
            bytecode.extend(self.compile_expr(expr, locals, code_buffer)?);

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
    fn compile_print(&mut self, elements: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut bytecode = Vec::new();

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
            bytecode.extend(Opcode::Getstatic(out_ref).encode());
            bytecode.extend(self.compile_expr(expr, locals, code_buffer)?);
            bytecode.extend(Opcode::Invokevirtual(println_ref).encode());
        }

        // Return nil
        bytecode.extend(Opcode::Aconst_null.encode());

        Ok(bytecode)
    }

    /// Compile arithmetic operations
    fn compile_arithmetic(&mut self, op: &str, args: &[Expr], locals: &mut HashMap<String, u8>, code_buffer: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        if args.is_empty() {
            return Err(format!("{} requires at least one argument", op));
        }

        let mut bytecode = Vec::new();

        // Compile first argument
        bytecode.extend(self.compile_expr(&args[0], locals, code_buffer)?);
        bytecode.extend(self.unbox_double()?);

        // Process remaining arguments
        for arg in &args[1..] {
            bytecode.extend(self.compile_expr(arg, locals, code_buffer)?);
            bytecode.extend(self.unbox_double()?);

            match op {
                "+" => bytecode.extend(Opcode::Dadd.encode()),
                "-" => bytecode.extend(Opcode::Dsub.encode()),
                "*" => bytecode.extend(Opcode::Dmul.encode()),
                "/" => bytecode.extend(Opcode::Ddiv.encode()),
                "mod" => bytecode.extend(Opcode::Drem.encode()),
                _ => {}
            }
        }

        // Handle comparison operators
        if matches!(op, "=" | "<" | ">" | "<=" | ">=") {
            bytecode.extend(Opcode::Dcmpl.encode());

            match op {
                "=" => {
                    bytecode.extend(Opcode::Ifeq(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                "<" => {
                    bytecode.extend(Opcode::Iflt(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                ">" => {
                    bytecode.extend(Opcode::Ifgt(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                "<=" => {
                    bytecode.extend(Opcode::Ifle(6).encode());
                    bytecode.extend(self.load_boolean(false)?);
                    bytecode.extend(Opcode::Goto(3).encode());
                    bytecode.extend(self.load_boolean(true)?);
                }
                ">=" => {
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

        let double_class = self.class.constant_pool.add_class("java/lang/Double".to_string());
        bytecode.extend(Opcode::Checkcast(double_class).encode());

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
        let valueof_ref = self.class.constant_pool.add_methodref(
            "java/lang/Double".to_string(),
            "valueOf".to_string(),
            "(D)Ljava/lang/Double;".to_string(),
        );
        Ok(Opcode::Invokestatic(valueof_ref).encode())
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
