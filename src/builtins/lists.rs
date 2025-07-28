use crate::environment::Environment;
use crate::value::Value;
use crate::error::EvalError;
use crate::tokenizer::Position;

pub fn register(env: &mut Environment) {
    env.define("list", Value::Function(builtin_list));
    env.define("car", Value::Function(builtin_car));
    env.define("cdr", Value::Function(builtin_cdr));
    env.define("cons", Value::Function(builtin_cons));
    env.define("length", Value::Function(builtin_length));
    env.define("null?", Value::Function(builtin_null));
    env.define("reverse", Value::Function(builtin_reverse));
}

fn builtin_list(args: &[Value]) -> Result<Value, EvalError> {
    Ok(Value::List(args.to_vec()))
}

fn builtin_car(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("car requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::List(list) => {
            if list.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(list[0].clone())
            }
        }
        _ => Err(EvalError::TypeError("car requires a list".to_string(), Position::new(1, 1))),
    }
}

fn builtin_cdr(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("cdr requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::List(list) => {
            if list.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::List(list[1..].to_vec()))
            }
        }
        _ => Err(EvalError::TypeError("cdr requires a list".to_string(), Position::new(1, 1))),
    }
}

fn builtin_cons(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("cons requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match &args[1] {
        Value::List(list) => {
            let mut new_list = vec![args[0].clone()];
            new_list.extend(list.iter().cloned());
            Ok(Value::List(new_list))
        }
        Value::Nil => Ok(Value::List(vec![args[0].clone()])),
        _ => Err(EvalError::TypeError("cons requires second argument to be a list".to_string(), Position::new(1, 1))),
    }
}

fn builtin_length(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("length requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::List(list) => Ok(Value::Number(list.len() as f64)),
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(EvalError::TypeError("length requires a list or string".to_string(), Position::new(1, 1))),
    }
}

fn builtin_null(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("null? requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    let result = match &args[0] {
        Value::Nil => true,
        Value::List(list) => list.is_empty(),
        _ => false,
    };
    
    Ok(Value::Boolean(result))
}

fn builtin_reverse(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("reverse requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::List(list) => {
            let mut reversed = list.clone();
            reversed.reverse();
            Ok(Value::List(reversed))
        }
        Value::Nil => Ok(Value::List(vec![])),
        _ => Err(EvalError::TypeError("reverse requires a list".to_string(), Position::new(1, 1))),
    }
}