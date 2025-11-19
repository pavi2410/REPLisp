/// Common compilation infrastructure shared across backends
///
/// This module contains data structures and utilities that are useful for
/// multiple code generation backends.

use crate::parser::{Expr, ExprType};
use std::collections::HashMap;

/// Information about a defined function
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// The name of the function in source code
    pub name: String,
    /// Number of parameters the function accepts
    pub param_count: usize,
    /// The function body (expressions to compile)
    pub body: Vec<Expr>,
    /// The parameter names
    pub params: Vec<String>,
}

/// Registry for tracking defined functions during compilation
#[derive(Debug, Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, FunctionInfo>,
}

impl FunctionRegistry {
    /// Create a new empty function registry
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Register a function
    pub fn register(&mut self, info: FunctionInfo) {
        self.functions.insert(info.name.clone(), info);
    }

    /// Look up a function by name
    pub fn get(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name)
    }

    /// Check if a function is defined
    pub fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Get all function names
    pub fn function_names(&self) -> Vec<&str> {
        self.functions.keys().map(|s| s.as_str()).collect()
    }

    /// Iterate over all functions
    pub fn iter(&self) -> impl Iterator<Item = (&String, &FunctionInfo)> {
        self.functions.iter()
    }
}

/// Extract function definitions from a list of expressions
///
/// Scans through expressions looking for `defn` forms and extracts their
/// information. This is useful for two-pass compilation where we need to
/// know about all functions before generating code.
///
/// # Arguments
/// * `exprs` - The list of expressions to scan
///
/// # Returns
/// A FunctionRegistry containing all found function definitions, or an error
pub fn extract_function_definitions(exprs: &[Expr]) -> Result<FunctionRegistry, String> {
    let mut registry = FunctionRegistry::new();

    for expr in exprs {
        if let ExprType::List(elements) = &expr.expr_type {
            if !elements.is_empty() {
                if let ExprType::Symbol(op) = &elements[0].expr_type {
                    if op == "defn" {
                        let info = parse_defn(elements)?;
                        registry.register(info);
                    }
                }
            }
        }
    }

    Ok(registry)
}

/// Parse a defn form into FunctionInfo
///
/// Expected form: (defn name [param1 param2 ...] body...)
///
/// # Arguments
/// * `elements` - The list elements of the defn form (including the "defn" symbol)
///
/// # Returns
/// FunctionInfo for the defined function, or an error
fn parse_defn(elements: &[Expr]) -> Result<FunctionInfo, String> {
    if elements.len() < 4 {
        return Err("defn requires at least 3 arguments: (defn name [params] body...)".to_string());
    }

    // Extract function name
    let name = match &elements[1].expr_type {
        ExprType::Symbol(s) => s.clone(),
        _ => return Err("defn requires a symbol as function name".to_string()),
    };

    // Extract parameters
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

    // Extract body (all remaining expressions)
    let body = elements[3..].to_vec();

    Ok(FunctionInfo {
        name,
        param_count: params.len(),
        body,
        params,
    })
}

/// Check if an expression is a special form
///
/// Special forms are built-in language constructs that have special
/// evaluation/compilation rules (e.g., if, def, defn, do, lambda, quote).
///
/// # Arguments
/// * `expr` - The expression to check
///
/// # Returns
/// Some(operator_name) if it's a special form, None otherwise
pub fn is_special_form(expr: &Expr) -> Option<&str> {
    match &expr.expr_type {
        ExprType::List(elements) if !elements.is_empty() => {
            match &elements[0].expr_type {
                ExprType::Symbol(op) => {
                    match op.as_str() {
                        "if" | "def" | "defn" | "do" | "lambda" | "quote" => Some(op),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Check if an expression is a builtin function call
///
/// # Arguments
/// * `expr` - The expression to check
///
/// # Returns
/// Some(function_name) if it's a builtin, None otherwise
pub fn is_builtin(expr: &Expr) -> Option<&str> {
    match &expr.expr_type {
        ExprType::List(elements) if !elements.is_empty() => {
            match &elements[0].expr_type {
                ExprType::Symbol(op) => {
                    match op.as_str() {
                        // Arithmetic
                        "+" | "-" | "*" | "/" | "mod" => Some(op),
                        // Comparison
                        "=" | "<" | ">" | "<=" | ">=" => Some(op),
                        // List operations
                        "list" | "car" | "cdr" | "cons" | "length" | "null?" | "reverse" => Some(op),
                        // I/O
                        "print" => Some(op),
                        // Logic
                        "and" | "or" | "not" => Some(op),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{tokenize, Position};
    use crate::parser::Parser;

    #[test]
    fn test_extract_function_definitions() {
        let source = "(defn add [x y] (+ x y)) (defn square [x] (* x x))";
        let tokens = tokenize(source).unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();

        let registry = extract_function_definitions(&exprs).unwrap();

        assert!(registry.contains("add"));
        assert!(registry.contains("square"));

        let add_info = registry.get("add").unwrap();
        assert_eq!(add_info.param_count, 2);
        assert_eq!(add_info.params, vec!["x", "y"]);

        let square_info = registry.get("square").unwrap();
        assert_eq!(square_info.param_count, 1);
        assert_eq!(square_info.params, vec!["x"]);
    }

    #[test]
    fn test_is_special_form() {
        let tokens = tokenize("(if true 1 2)").unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();

        assert_eq!(is_special_form(&exprs[0]), Some("if"));

        let tokens = tokenize("(+ 1 2)").unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();

        assert_eq!(is_special_form(&exprs[0]), None);
    }

    #[test]
    fn test_is_builtin() {
        let tokens = tokenize("(+ 1 2)").unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();

        assert_eq!(is_builtin(&exprs[0]), Some("+"));

        let tokens = tokenize("(custom-fn 1 2)").unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();

        assert_eq!(is_builtin(&exprs[0]), None);
    }
}
