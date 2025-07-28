use crate::parser::Expr;
use crate::environment::Environment;
use crate::error::EvalError;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Symbol(String),
    Boolean(bool),
    List(Vec<Value>),
    Function(fn(&[Value]) -> Result<Value, EvalError>),
    Lambda {
        params: Vec<String>,
        body: Vec<Expr>,
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
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false, // Functions and lambdas are not comparable
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
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