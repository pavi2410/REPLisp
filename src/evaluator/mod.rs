use crate::parser::{Expr, ExprType};
use crate::value::Value;
use crate::error::{EvalError, StackFrame};
use crate::environment::Environment;

pub mod special_forms;
pub mod function_calls;
pub mod quote;

pub fn eval_expr(expr: &Expr, env: &mut Environment) -> Result<Value, EvalError> {
    eval_expr_with_stack(expr, env, &mut Vec::new())
}

pub fn eval_expr_with_stack(
    expr: &Expr, 
    env: &mut Environment, 
    stack: &mut Vec<StackFrame>
) -> Result<Value, EvalError> {
    match &expr.expr_type {
        ExprType::Number(n) => Ok(Value::Number(*n)),
        ExprType::String(s) => Ok(Value::String(s.clone())),
        ExprType::Symbol(s) => eval_symbol(s, env, expr),
        ExprType::Quote(inner) => quote::eval_quote(inner),
        ExprType::List(elements) => eval_list(elements, env, stack),
    }
}

fn eval_symbol(symbol: &str, env: &Environment, expr: &Expr) -> Result<Value, EvalError> {
    match symbol {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        _ => env.lookup(symbol)
            .cloned()
            .ok_or_else(|| EvalError::UndefinedSymbol(symbol.to_string(), expr.position.clone()))
    }
}

fn eval_list(
    elements: &[Expr], 
    env: &mut Environment, 
    stack: &mut Vec<StackFrame>
) -> Result<Value, EvalError> {
    if elements.is_empty() {
        return Ok(Value::List(vec![]));
    }

    // Check for special forms
    if let ExprType::Symbol(name) = &elements[0].expr_type {
        match name.as_str() {
            "def" => special_forms::eval_def(&elements[1..], env, stack),
            "defn" => special_forms::eval_defn(&elements[1..], env, stack),
            "lambda" => special_forms::eval_lambda(&elements[1..], env, stack),
            "if" => special_forms::eval_if(&elements[1..], env, stack),
            "cond" => special_forms::eval_cond(&elements[1..], env, stack),
            "do" => special_forms::eval_do(&elements[1..], env, stack),
            _ => function_calls::eval_function_call(elements, env, stack),
        }
    } else {
        function_calls::eval_function_call(elements, env, stack)
    }
}