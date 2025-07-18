use crate::parser::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Value>),
    Function(fn(&[Value]) -> Result<Value, EvalError>),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
        closure: Environment,
    },
    Nil,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false, // Functions and lambdas are not comparable
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    UndefinedSymbol(String),
    TypeError(String),
    ArityError(String),
    DivisionByZero,
    InvalidFunction(String),
}

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            bindings: HashMap::new(),
        };
        
        // Add built-in functions
        env.define("+", Value::Function(builtin_add));
        env.define("-", Value::Function(builtin_subtract));
        env.define("*", Value::Function(builtin_multiply));
        env.define("/", Value::Function(builtin_divide));
        env.define("=", Value::Function(builtin_equal));
        env.define("<", Value::Function(builtin_less_than));
        env.define(">", Value::Function(builtin_greater_than));
        env.define("list", Value::Function(builtin_list));
        env.define("car", Value::Function(builtin_car));
        env.define("cdr", Value::Function(builtin_cdr));
        env.define("cons", Value::Function(builtin_cons));
        env.define("length", Value::Function(builtin_length));
        env.define("null?", Value::Function(builtin_null));
        env.define("print", Value::Function(builtin_print));
        env.define("min", Value::Function(builtin_min));
        env.define("max", Value::Function(builtin_max));
        env.define("abs", Value::Function(builtin_abs));
        env.define("mod", Value::Function(builtin_mod));
        
        env
    }
    
    pub fn define(&mut self, name: &str, value: Value) {
        self.bindings.insert(name.to_string(), value);
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Value> {
        self.bindings.get(name)
    }
}

// Built-in arithmetic functions
fn builtin_add(args: &[Value]) -> Result<Value, EvalError> {
    let mut sum = 0.0;
    for arg in args {
        match arg {
            Value::Number(n) => sum += n,
            _ => return Err(EvalError::TypeError("+ requires numbers".to_string())),
        }
    }
    Ok(Value::Number(sum))
}

fn builtin_subtract(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("- requires at least 1 argument".to_string()));
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
                        _ => return Err(EvalError::TypeError("- requires numbers".to_string())),
                    }
                }
                Ok(Value::Number(result))
            }
        }
        _ => Err(EvalError::TypeError("- requires numbers".to_string())),
    }
}

fn builtin_multiply(args: &[Value]) -> Result<Value, EvalError> {
    let mut product = 1.0;
    for arg in args {
        match arg {
            Value::Number(n) => product *= n,
            _ => return Err(EvalError::TypeError("* requires numbers".to_string())),
        }
    }
    Ok(Value::Number(product))
}

fn builtin_divide(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("/ requires at least 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::Number(first) => {
            if args.len() == 1 {
                if *first == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(Value::Number(1.0 / first))
            } else {
                let mut result = *first;
                for arg in &args[1..] {
                    match arg {
                        Value::Number(n) => {
                            if *n == 0.0 {
                                return Err(EvalError::DivisionByZero);
                            }
                            result /= n;
                        }
                        _ => return Err(EvalError::TypeError("/ requires numbers".to_string())),
                    }
                }
                Ok(Value::Number(result))
            }
        }
        _ => Err(EvalError::TypeError("/ requires numbers".to_string())),
    }
}

fn builtin_equal(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("= requires exactly 2 arguments".to_string()));
    }
    
    let result = match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Symbol(a), Value::Symbol(b)) => a == b,
        _ => false,
    };
    
    Ok(Value::Number(if result { 1.0 } else { 0.0 }))
}

fn builtin_less_than(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("< requires exactly 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(if a < b { 1.0 } else { 0.0 }))
        }
        _ => Err(EvalError::TypeError("< requires numbers".to_string())),
    }
}

fn builtin_greater_than(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("> requires exactly 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(if a > b { 1.0 } else { 0.0 }))
        }
        _ => Err(EvalError::TypeError("> requires numbers".to_string())),
    }
}

fn builtin_list(args: &[Value]) -> Result<Value, EvalError> {
    Ok(Value::List(args.to_vec()))
}

fn builtin_car(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("car requires exactly 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::List(list) => {
            if list.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(list[0].clone())
            }
        }
        _ => Err(EvalError::TypeError("car requires a list".to_string())),
    }
}

fn builtin_cdr(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("cdr requires exactly 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::List(list) => {
            if list.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::List(list[1..].to_vec()))
            }
        }
        _ => Err(EvalError::TypeError("cdr requires a list".to_string())),
    }
}

fn builtin_cons(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("cons requires exactly 2 arguments".to_string()));
    }
    
    match &args[1] {
        Value::List(list) => {
            let mut new_list = vec![args[0].clone()];
            new_list.extend(list.iter().cloned());
            Ok(Value::List(new_list))
        }
        Value::Nil => Ok(Value::List(vec![args[0].clone()])),
        _ => Err(EvalError::TypeError("cons requires second argument to be a list".to_string())),
    }
}

fn builtin_length(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("length requires exactly 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::List(list) => Ok(Value::Number(list.len() as f64)),
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(EvalError::TypeError("length requires a list or string".to_string())),
    }
}

fn builtin_null(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("null? requires exactly 1 argument".to_string()));
    }
    
    let result = match &args[0] {
        Value::Nil => true,
        Value::List(list) => list.is_empty(),
        _ => false,
    };
    
    Ok(Value::Number(if result { 1.0 } else { 0.0 }))
}

fn builtin_print(args: &[Value]) -> Result<Value, EvalError> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        match arg {
            Value::String(s) => print!("{}", s),  // Print strings without quotes
            other => print!("{}", other),
        }
    }
    println!();  // Add newline
    Ok(Value::Nil)
}

fn builtin_min(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("min requires at least 1 argument".to_string()));
    }
    
    let mut min_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(EvalError::TypeError("min requires numbers".to_string())),
    };
    
    for arg in &args[1..] {
        match arg {
            Value::Number(n) => {
                if *n < min_val {
                    min_val = *n;
                }
            }
            _ => return Err(EvalError::TypeError("min requires numbers".to_string())),
        }
    }
    
    Ok(Value::Number(min_val))
}

fn builtin_max(args: &[Value]) -> Result<Value, EvalError> {
    if args.is_empty() {
        return Err(EvalError::ArityError("max requires at least 1 argument".to_string()));
    }
    
    let mut max_val = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(EvalError::TypeError("max requires numbers".to_string())),
    };
    
    for arg in &args[1..] {
        match arg {
            Value::Number(n) => {
                if *n > max_val {
                    max_val = *n;
                }
            }
            _ => return Err(EvalError::TypeError("max requires numbers".to_string())),
        }
    }
    
    Ok(Value::Number(max_val))
}

fn builtin_abs(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError("abs requires exactly 1 argument".to_string()));
    }
    
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(EvalError::TypeError("abs requires a number".to_string())),
    }
}

fn builtin_mod(args: &[Value]) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("mod requires exactly 2 arguments".to_string()));
    }
    
    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => {
            if *b == 0.0 {
                return Err(EvalError::DivisionByZero);
            }
            Ok(Value::Number(a % b))
        }
        _ => Err(EvalError::TypeError("mod requires numbers".to_string())),
    }
}

pub fn eval_expr(expr: &Expr, env: &mut Environment) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Symbol(s) => {
            env.lookup(s)
                .cloned()
                .ok_or_else(|| EvalError::UndefinedSymbol(s.clone()))
        }
        Expr::Quote(expr) => eval_quote(expr),
        Expr::List(elements) => {
            if elements.is_empty() {
                Ok(Value::List(vec![]))
            } else {
                // Check for special forms
                if let Expr::Symbol(name) = &elements[0] {
                    match name.as_str() {
                        "def" => eval_def(&elements[1..], env),
                        "defn" => eval_defn(&elements[1..], env),
                        "lambda" => eval_lambda(&elements[1..], env),
                        _ => eval_function_call(elements, env),
                    }
                } else {
                    eval_function_call(elements, env)
                }
            }
        }
    }
}

fn eval_quote(expr: &Expr) -> Result<Value, EvalError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Symbol(s) => Ok(Value::Symbol(s.clone())),
        Expr::List(elements) => {
            let mut values = Vec::new();
            for elem in elements {
                values.push(eval_quote(elem)?);
            }
            Ok(Value::List(values))
        }
        Expr::Quote(expr) => eval_quote(expr),
    }
}

fn eval_def(args: &[Expr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("def requires exactly 2 arguments".to_string()));
    }
    
    let name = match &args[0] {
        Expr::Symbol(s) => s.clone(),
        _ => return Err(EvalError::TypeError("def requires a symbol as first argument".to_string())),
    };
    
    let value = eval_expr(&args[1], env)?;
    env.define(&name, value.clone());
    Ok(value)
}

fn eval_defn(args: &[Expr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::ArityError("defn requires exactly 3 arguments".to_string()));
    }
    
    let name = match &args[0] {
        Expr::Symbol(s) => s.clone(),
        _ => return Err(EvalError::TypeError("defn requires a symbol as first argument".to_string())),
    };
    
    let params = match &args[1] {
        Expr::List(param_exprs) => {
            let mut params = Vec::new();
            for param_expr in param_exprs {
                match param_expr {
                    Expr::Symbol(s) => params.push(s.clone()),
                    _ => return Err(EvalError::TypeError("defn parameters must be symbols".to_string())),
                }
            }
            params
        }
        _ => return Err(EvalError::TypeError("defn requires a parameter list as second argument".to_string())),
    };
    
    let body = args[2].clone();
    
    let lambda = Value::Lambda {
        params,
        body: Box::new(body),
        closure: env.clone(),
    };
    
    env.define(&name, lambda.clone());
    Ok(lambda)
}

fn eval_lambda(args: &[Expr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError("lambda requires exactly 2 arguments".to_string()));
    }
    
    let params = match &args[0] {
        Expr::List(param_exprs) => {
            let mut params = Vec::new();
            for param_expr in param_exprs {
                match param_expr {
                    Expr::Symbol(s) => params.push(s.clone()),
                    _ => return Err(EvalError::TypeError("lambda parameters must be symbols".to_string())),
                }
            }
            params
        }
        _ => return Err(EvalError::TypeError("lambda requires a parameter list as first argument".to_string())),
    };
    
    let body = args[1].clone();
    
    Ok(Value::Lambda {
        params,
        body: Box::new(body),
        closure: env.clone(),
    })
}

fn eval_function_call(elements: &[Expr], env: &mut Environment) -> Result<Value, EvalError> {
    if elements.is_empty() {
        return Ok(Value::List(vec![]));
    }
    
    let func_expr = &elements[0];
    let args_exprs = &elements[1..];
    
    // Evaluate function
    let func = eval_expr(func_expr, env)?;
    
    // Evaluate arguments
    let mut args = Vec::new();
    for arg_expr in args_exprs {
        args.push(eval_expr(arg_expr, env)?);
    }
    
    // Call function
    match func {
        Value::Function(f) => f(&args),
        Value::Lambda { params, body, mut closure } => {
            // Check arity
            if args.len() != params.len() {
                return Err(EvalError::ArityError(format!(
                    "Function expects {} arguments, got {}",
                    params.len(),
                    args.len()
                )));
            }
            
            // Bind arguments to parameters in closure environment
            for (param, arg) in params.iter().zip(args.iter()) {
                closure.define(param, arg.clone());
            }
            
            // Evaluate body in closure environment
            eval_expr(&body, &mut closure)
        }
        _ => Err(EvalError::InvalidFunction(format!("Not a function: {:?}", func))),
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::List(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
            Value::Function(_) => write!(f, "<function>"),
            Value::Lambda { params, .. } => {
                write!(f, "<lambda (")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")>")
            }
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::UndefinedSymbol(s) => write!(f, "Undefined symbol: {}", s),
            EvalError::TypeError(msg) => write!(f, "Type error: {}", msg),
            EvalError::ArityError(msg) => write!(f, "Arity error: {}", msg),
            EvalError::DivisionByZero => write!(f, "Division by zero"),
            EvalError::InvalidFunction(msg) => write!(f, "Invalid function: {}", msg),
        }
    }
}

impl std::error::Error for EvalError {}