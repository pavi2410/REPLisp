use crate::environment::Environment;
use crate::value::Value;
use crate::error::EvalError;
use crate::tokenizer::Position;

pub fn register(env: &mut Environment) {
    env.define("not", Value::Function(builtin_not));
    env.define("and", Value::Function(builtin_and));
    env.define("or", Value::Function(builtin_or));
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Boolean(b) => *b,
        Value::Number(n) => *n != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::List(list) => !list.is_empty(),
        _ => true, // Functions, symbols, and other values are truthy
    }
}

fn builtin_not(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("not requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    let result = !is_truthy(&args[0]);
    Ok(Value::Boolean(result))
}

fn builtin_and(args: &[Value]) -> Result<Value, EvalError> {
    // Short-circuiting: return false if any argument is falsy
    for arg in args {
        if !is_truthy(arg) {
            return Ok(Value::Boolean(false));
        }
    }
    // All arguments are truthy
    Ok(Value::Boolean(true))
}

fn builtin_or(args: &[Value]) -> Result<Value, EvalError> {
    // Short-circuiting: return true if any argument is truthy
    for arg in args {
        if is_truthy(arg) {
            return Ok(Value::Boolean(true));
        }
    }
    // All arguments are falsy
    Ok(Value::Boolean(false))
}