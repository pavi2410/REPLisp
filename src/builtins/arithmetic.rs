use crate::environment::Environment;
use crate::value::Value;
use crate::error::EvalError;
use crate::tokenizer::Position;

pub fn register(env: &mut Environment) {
    env.define("+", Value::Function(builtin_add));
    env.define("-", Value::Function(builtin_subtract));
    env.define("*", Value::Function(builtin_multiply));
    env.define("/", Value::Function(builtin_divide));
    env.define("=", Value::Function(builtin_equal));
    env.define("<", Value::Function(builtin_less_than));
    env.define(">", Value::Function(builtin_greater_than));
    env.define("<=", Value::Function(builtin_less_than_or_equal));
    env.define(">=", Value::Function(builtin_greater_than_or_equal));
    env.define("min", Value::Function(builtin_min));
    env.define("max", Value::Function(builtin_max));
    env.define("abs", Value::Function(builtin_abs));
    env.define("mod", Value::Function(builtin_mod));
}

fn builtin_add(args: &[Value]) -> Result<Value, EvalError> {
    let mut sum = 0.0;
    for arg in args {
        match arg {
            Value::Number(n) => sum += n,
            _ => return Err(EvalError::TypeError("+ requires numbers".to_string(), Position::new(1, 1))),
        }
    }
    Ok(Value::Number(sum))
}

fn builtin_subtract(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("- requires at least 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::Number(first) => {
            if args.len() == 1 {
                Ok(Value::Number(-first))
            } else {
                let mut result = *first;
                for arg in &args[1..] {
                    match arg {
                        Value::Number(n) => result -= n,
                        _ => return Err(EvalError::TypeError("- requires numbers".to_string(), Position::new(1, 1))),
                    }
                }
                Ok(Value::Number(result))
            }
        }
        _ => Err(EvalError::TypeError("- requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_multiply(args: &[Value]) -> Result<Value, EvalError> {
    let mut product = 1.0;
    for arg in args {
        match arg {
            Value::Number(n) => product *= n,
            _ => return Err(EvalError::TypeError("* requires numbers".to_string(), Position::new(1, 1))),
        }
    }
    Ok(Value::Number(product))
}

fn builtin_divide(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("/ requires at least 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::Number(first) => {
            if args.len() == 1 {
                if *first == 0.0 {
                    return Err(EvalError::DivisionByZero(Position::new(1, 1)));
                }
                Ok(Value::Number(1.0 / first))
            } else {
                let mut result = *first;
                for arg in &args[1..] {
                    match arg {
                        Value::Number(n) => {
                            if *n == 0.0 {
                                return Err(EvalError::DivisionByZero(Position::new(1, 1)));
                            }
                            result /= n;
                        }
                        _ => return Err(EvalError::TypeError("/ requires numbers".to_string(), Position::new(1, 1))),
                    }
                }
                Ok(Value::Number(result))
            }
        }
        _ => Err(EvalError::TypeError("/ requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_equal(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("= requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    let result = match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Symbol(a), Value::Symbol(b)) => a == b,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Nil, Value::Nil) => true,
        _ => false,
    };
    
    Ok(Value::Boolean(result))
}

fn builtin_less_than(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("< requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Boolean(a < b))
        }
        _ => Err(EvalError::TypeError("< requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_greater_than(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("> requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Boolean(a > b))
        }
        _ => Err(EvalError::TypeError("> requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_less_than_or_equal(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("<= requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Boolean(a <= b))
        }
        _ => Err(EvalError::TypeError("<= requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_greater_than_or_equal(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError(">= requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Boolean(a >= b))
        }
        _ => Err(EvalError::TypeError(">= requires numbers".to_string(), Position::new(1, 1))),
    }
}

fn builtin_min(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("min requires at least 1 argument".to_string(), Position::new(1, 1)));
    }
    
    let mut min_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(EvalError::TypeError("min requires numbers".to_string(), Position::new(1, 1))),
    };
    
    for arg in &args[1..] {
        match arg {
            Value::Number(n) => {
                if *n < min_val {
                    min_val = *n;
                }
            }
            _ => return Err(EvalError::TypeError("min requires numbers".to_string(), Position::new(1, 1))),
        }
    }
    
    Ok(Value::Number(min_val))
}

fn builtin_max(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("max requires at least 1 argument".to_string(), Position::new(1, 1)));
    }
    
    let mut max_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(EvalError::TypeError("max requires numbers".to_string(), Position::new(1, 1))),
    };
    
    for arg in &args[1..] {
        match arg {
            Value::Number(n) => {
                if *n > max_val {
                    max_val = *n;
                }
            }
            _ => return Err(EvalError::TypeError("max requires numbers".to_string(), Position::new(1, 1))),
        }
    }
    
    Ok(Value::Number(max_val))
}

fn builtin_abs(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("abs requires exactly 1 argument".to_string(), Position::new(1, 1)));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(EvalError::TypeError("abs requires a number".to_string(), Position::new(1, 1))),
    }
}

fn builtin_mod(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("mod requires exactly 2 arguments".to_string(), Position::new(1, 1)));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
                return Err(EvalError::DivisionByZero(Position::new(1, 1)));
            }
            Ok(Value::Number(a % b))
        }
        _ => Err(EvalError::TypeError("mod requires numbers".to_string(), Position::new(1, 1))),
    }
}