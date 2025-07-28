use crate::parser::{Expr, ExprType};
use crate::value::Value;
use crate::error::EvalError;

pub fn eval_quote(expr: &Expr) -> Result<Value, EvalError> {
    match &expr.expr_type {
        ExprType::Number(n) => Ok(Value::Number(*n)),
        ExprType::String(s) => Ok(Value::String(s.clone())),
        ExprType::Symbol(s) => {
            match s.as_str() {
                "true" => Ok(Value::Boolean(true)),
                "false" => Ok(Value::Boolean(false)),
                _ => Ok(Value::Symbol(s.clone()))
            }
        }
        ExprType::List(elements) => {
            let mut values = Vec::new();
            for elem in elements {
                values.push(eval_quote(elem)?);
            }
            Ok(Value::List(values))
        }
        ExprType::Quote(inner_expr) => eval_quote(inner_expr),
    }
}