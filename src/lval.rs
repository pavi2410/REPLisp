use crate::error::{Error, ReplispResult};
use std::collections::HashMap;
use std::fmt;

type LvalChildren = Vec<Box<Lval>>;
pub type LBuiltin = fn(&mut Lval) -> Result<Box<Lval>, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    /// A builtin function
    ///
    /// 1. The name of the function
    /// 2. The function to call
    Builtin(String, LBuiltin),
    /// A user-defined function
    ///
    /// 1. Environment
    /// 2. Formal arguments
    /// 3. Body
    Lambda(HashMap<String, Box<Lval>>, Box<Lval>, Box<Lval>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lval {
    Num(i64),
    Sym(String),
    Str(String), // Added string literal variant
    Sexpr(LvalChildren),
    Qexpr(LvalChildren),
    Fun(Func),
}

impl Lval {
    pub fn as_num(&self) -> ReplispResult<i64> {
        match *self {
            Lval::Num(n_num) => Ok(n_num),
            _ => Err(Error::NotANumber),
        }
    }
    pub fn as_string(&self) -> ReplispResult<String> {
        match self {
            Lval::Sym(s) => Ok(s.to_string()),
            Lval::Str(s) => Ok(s.to_string()),
            _ => Err(Error::WrongType(
                "string or symbol".to_string(),
                format!("{self}"),
            )),
        }
    }
    pub fn len(&self) -> ReplispResult<usize> {
        match self {
            Lval::Sexpr(children) | Lval::Qexpr(children) => {
                Ok(children.len())
            }
            _ => Err(Error::NoChildren),
        }
    }
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lval::Num(n) => write!(f, "{n}"),
            Lval::Sym(s) => write!(f, "{s}"),
            Lval::Str(s) => write!(f, "\"{s}\""),
            Lval::Sexpr(cell) => write!(f, "({})", lval_expr_print(cell)),
            Lval::Qexpr(cell) => write!(f, "{{{}}}", lval_expr_print(cell)),
            Lval::Fun(func) => match func {
                Func::Builtin(name, _) => write!(f, "<builtin {}>", name),
                Func::Lambda(_, formals, body) => write!(f, "(\\ {formals} {body})"),
            },
        }
    }
}

fn lval_expr_print(cell: &[Box<Lval>]) -> String {
    let mut ret = String::new();
    for i in 0..cell.len() {
        ret.push_str(&format!("{}", cell[i]));
        if i < cell.len() - 1 {
            ret.push(' ');
        }
    }
    ret
}

// Constructors
// Each allocates a brand new boxed Lval
// The recursive types start empty

pub fn builtin(f: LBuiltin, name: &str) -> Box<Lval> {
    Box::new(Lval::Fun(Func::Builtin(name.to_string(), f)))
}

pub fn lambda(env: HashMap<String, Box<Lval>>, formals: Box<Lval>, body: Box<Lval>) -> Box<Lval> {
    Box::new(Lval::Fun(Func::Lambda(env, formals, body)))
}

pub fn num(n: i64) -> Box<Lval> {
    Box::new(Lval::Num(n))
}

pub fn sexpr() -> Box<Lval> {
    Box::new(Lval::Sexpr(Vec::new()))
}

pub fn qexpr() -> Box<Lval> {
    Box::new(Lval::Qexpr(Vec::new()))
}

// Manipulating children

// Add lval x to lval::sexpr or lval::qexpr v
pub fn add(v: &mut Lval, x: &Lval) -> ReplispResult<()> {
    match *v {
        Lval::Sexpr(ref mut children) | Lval::Qexpr(ref mut children) => {
            children.push(Box::new(x.clone()));
        }
        _ => return Err(Error::NoChildren),
    }
    Ok(())
}

// Extract single element of sexpr at index i
pub fn pop(v: &mut Lval, i: usize) -> ReplispResult<Box<Lval>> {
    match *v {
        Lval::Sexpr(ref mut children) | Lval::Qexpr(ref mut children) => {
            let ret = children[i].clone();
            children.remove(i);
            Ok(ret)
        }
        _ => Err(Error::NoChildren),
    }
}

// Add each cell in y to x
pub fn join(x: &mut Lval, mut y: Box<Lval>) -> ReplispResult<()> {
    while y.len()? > 0 {
        add(x, &*pop(&mut y, 0)?)?;
    }
    Ok(())
}
