use crate::error::{Error, ReplispResult};
use crate::lenv::Lenv;
use crate::lval::{add, builtin, join, lambda, num, pop, qexpr, sexpr, Func, LBuiltin, Lval};
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Rem, Sub};

// macro to shorten code for applying a binary operation to two Lvals
macro_rules! apply_binop {
    ( $op:ident, $x:ident, $y:ident ) => {
        match (*$x, *$y) {
            (Lval::Num(x_num), Lval::Num(y_num)) => {
                $x = num(x_num.$op(y_num));
                continue;
            }
            _ => return Err(Error::NotANumber),
        }
    };
}

// apply a binary operation {+ - * / ^ % min max} to a list of arguments in succession
fn builtin_op(v: &mut Lval, func: &str) -> ReplispResult<Box<Lval>> {
    let mut child_count = match *v {
        Lval::Sexpr(ref children) => children.len(),
        _ => return Ok(Box::new(v.clone())),
    };

    let mut x = pop(v, 0)?;

    // If no args given and we're doing subtraction, perform unary negation
    if (func == "-" || func == "sub") && child_count == 1 {
        println!("builtin_op: Unary negation on {}", x);
        let x_num = x.as_num()?;
        return Ok(num(-x_num));
    }

    // consume the children until empty
    // and operate on x
    while child_count > 1 {
        let y = pop(v, 0)?;
        child_count -= 1;
        match func {
            "+" | "add" => {
                println!("builtin_op: Add {} and {}", x, y);
                apply_binop!(add, x, y);
            }
            "-" | "sub" => {
                println!("builtin_op: Subtract {} and {}", x, y);
                apply_binop!(sub, x, y);
            }
            "*" | "mul" => {
                println!("builtin_op: Multiply {} and {}", x, y);
                apply_binop!(mul, x, y);
            }
            "/" | "div" => {
                if y.as_num()? == 0 {
                    println!("builtin_op: Failed divide {} by {}", x, y);
                    return Err(Error::DivideByZero);
                };
                println!("builtin_op: Divide {} by {}", x, y);
                apply_binop!(div, x, y);
            }
            "%" | "rem" => {
                println!("builtin_op: {} % {}", x, y);
                apply_binop!(rem, x, y);
            }
            "^" | "pow" => {
                println!("builtin_op: Raise {} to the {} power", x, y);
                let y_num = y.as_num()?;
                let x_num = x.as_num()?;
                let mut coll = 1;
                for _ in 0..y_num {
                    coll *= x_num;
                }
                x = num(coll);
            }
            "min" => {
                println!("builtin_op: Min {} and {}", x, y);
                let x_num = x.as_num()?;
                let y_num = y.as_num()?;
                if x_num < y_num {
                    x = num(x_num);
                } else {
                    x = num(y_num);
                };
            }
            "max" => {
                println!("builtin_op: Max {} and {}", x, y);
                let x_num = x.as_num()?;
                let y_num = y.as_num()?;
                if x_num > y_num {
                    x = num(x_num);
                } else {
                    x = num(y_num);
                };
            }
            _ => unreachable!(),
        }
    }
    Ok(x)
}

// Operator aliases, function pointers will be stored in env
// TODO macro??  create_builtin!(a, &str)
pub fn builtin_add(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "+")
}

pub fn builtin_sub(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "-")
}

pub fn builtin_mul(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "*")
}

pub fn builtin_div(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "/")
}

pub fn builtin_pow(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "^")
}

pub fn builtin_rem(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "%")
}

pub fn builtin_max(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "max")
}

pub fn builtin_min(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    builtin_op(a, "min")
}

/// Print values to the console
pub fn builtin_print(a: &mut Lval) -> ReplispResult<Box<Lval>> {
    let mut output = String::new();

    // Concatenate all arguments
    for _i in 0..a.len()? {
        let val = pop(a, 0)?;
        match *val {
            Lval::Sym(ref s) => {
                output.push_str(s);
            }
            Lval::Str(ref s) => {
                // For string literals, just output the string value
                output.push_str(s);
            }
            Lval::Sexpr(ref cells) => {
                // Handle S-expressions by evaluating them first
                if !cells.is_empty() {
                    output.push_str(&format!("{:?}", val));
                }
            }
            _ => output.push_str(&format!("{:?}", val)),
        }
    }

    println!("{}", output);
    Ok(sexpr())
}

/// Attach a value to the front of a qexpr
pub fn builtin_cons(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let child_count = v.len()?;
    if child_count != 2 {
        return Err(Error::NumArguments(2, child_count));
    }
    let new_elem = pop(v, 0)?;
    let maybe_qexpr = pop(v, 0)?;
    match *maybe_qexpr {
        Lval::Qexpr(ref children) => {
            let mut ret = qexpr();
            add(&mut ret, &new_elem)?;
            for c in children {
                add(&mut ret, &c.clone())?;
            }
            Ok(ret)
        }
        _ => Err(Error::WrongType("qexpr".to_string(), format!("{v:?}"))),
    }
}

/// terminate the program (or exit the prompt)
pub fn builtin_exit(_v: &mut Lval) -> ReplispResult<Box<Lval>> {
    // always succeeds
    println!("Goodbye!");
    ::std::process::exit(0);
}

/// Return the first element of a qexpr
pub fn builtin_head(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let mut qexpr = pop(v, 0)?;
    match *qexpr {
        Lval::Qexpr(ref mut children) => {
            if children.is_empty() {
                return Err(Error::EmptyList);
            }
            println!("builtin_head: Returning the first element");
            Ok(children[0].clone())
        }
        _ => Err(Error::WrongType("qexpr".to_string(), format!("{qexpr:?}"))),
    }
}

/// Return everything but the last element of a qexpr
pub fn builtin_init(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let maybe_qexpr = pop(v, 0)?;
    if let Lval::Qexpr(ref children) = *maybe_qexpr {
        let mut ret = qexpr();
        for item in children.iter().take(children.len() - 1) {
            add(&mut ret, &item.clone())?;
        }
        Ok(ret)
    } else {
        Err(Error::WrongType(
            "qexpr".to_string(),
            format!("{maybe_qexpr:?}"),
        ))
    }
}

/// Join the children into one qexpr
pub fn builtin_join(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let mut ret = qexpr();
    for _ in 0..v.len()? {
        let next = pop(v, 0)?;
        match *next {
            Lval::Qexpr(_) => {
                join(&mut ret, next)?;
            }
            _ => return Err(Error::WrongType("qexpr".to_string(), format!("{next:?}"))),
        }
    }
    Ok(ret)
}

/// builtin_lambda returns a lambda lval from two lists of symbols
pub fn builtin_lambda(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    // ensure there's only two arguments
    let child_count = v.len()?;
    if child_count != 2 {
        return Err(Error::NumArguments(2, child_count));
    }

    // first qexpr should contain only symbols - lval.as_string().is_ok()
    let formals = pop(v, 0)?;
    let formals_ret = formals.clone(); // ewwww but it gets moved on me?!  this might be why Rc<> - it doesn't need to mutate
    let body = pop(v, 0)?;
    match *formals {
        Lval::Qexpr(contents) => {
            for cell in contents {
                if cell.as_string().is_err() {
                    return Err(Error::WrongType("Symbol".to_string(), format!("{cell:?}")));
                }
            }
            match *body {
                Lval::Qexpr(_) => Ok(lambda(HashMap::new(), formals_ret, body)),
                _ => Err(Error::WrongType(
                    "Q-Expression".to_string(),
                    format!("{body:?}"),
                )),
            }
        }
        _ => Err(Error::WrongType(
            "Q-Expression".to_string(),
            format!("{formals:?}"),
        )),
    }
}

/// make sexpr into a qexpr
pub fn builtin_list(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    match *v {
        Lval::Sexpr(ref children) => {
            println!("builtin_list: Building qexpr from {:?}", children);
            let mut new_qexpr = qexpr();
            for c in children {
                let cloned = Box::new(*c.clone());
                add(&mut new_qexpr, &cloned)?;
            }
            Ok(new_qexpr)
        }
        _ => Ok(Box::new(v.clone())),
    }
}

pub fn builtin_len(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let child_count = v.len()?;
    match child_count {
        1 => {
            let qexpr = pop(v, 0)?;
            match *qexpr {
                Lval::Qexpr(_) => {
                    println!("Returning length of {qexpr:?}");
                    Ok(num(qexpr.len()? as i64))
                }
                _ => Err(Error::WrongType("qexpr".to_string(), format!("{qexpr:?}"))),
            }
        }
        _ => Err(Error::NumArguments(1, child_count)),
    }
}

/// Return all but the first element of a qexpr
pub fn builtin_tail(v: &mut Lval) -> ReplispResult<Box<Lval>> {
    let mut maybe_qexpr = pop(v, 0)?;
    println!("Returning tail of {:?}", maybe_qexpr);
    if let Lval::Qexpr(ref mut children) = *maybe_qexpr {
        if children.is_empty() {
            return Err(Error::EmptyList);
        }
        let mut ret = qexpr();
        for c in &children[1..] {
            add(&mut ret, &c.clone())?;
        }
        Ok(ret)
    } else {
        Err(Error::WrongType(
            "qexpr".to_string(),
            format!("{maybe_qexpr:?}"),
        ))
    }
}

pub fn lval_call(lenv: &mut Lenv, f: Lval, args: &mut Lval) -> ReplispResult<Box<Lval>> {
    match f {
        Lval::Fun(func) => match func {
            Func::Builtin(name, func) => {
                println!("Calling builtin function {}", name);

                // Special case for the 'do' function which needs the environment
                if name == "do" {
                    return builtin_do(lenv, args);
                }

                func(args)
            }
            Func::Lambda(env, formals, body) => {
                println!("Executing lambda");

                // bind arguments to parameters
                let mut local_env = Lenv::new(Some(env), None);

                // Bind arguments to formal parameters
                if let Lval::Qexpr(formals_vec) = *formals {
                    for (_i, formal) in formals_vec.iter().enumerate() {
                        if let Lval::Sym(name) = &**formal {
                            let val = pop(args, 0)?;
                            local_env.put(name.clone(), val);
                        } else {
                            return Err(Error::WrongType(
                                "Symbol".to_string(),
                                format!("{:?}", formal),
                            ));
                        }
                    }
                }

                // Evaluate the body in the new environment
                let mut body_clone = body.clone();
                lval_eval(&mut local_env, &mut body_clone)
            }
        },
        _ => Err(Error::WrongType("Function".to_owned(), format!("{f:?}"))),
    }
}

fn eval_cells(e: &mut Lenv, cells: &[Box<Lval>]) -> ReplispResult<Box<Lval>> {
    cells.iter().fold(Ok(sexpr()), |acc, c| {
        match acc {
            Ok(mut lval) => {
                add(&mut lval, &*lval_eval(e, &mut c.clone())?)?;
                Ok(lval)
            }
            // Handle errors properly instead of using unreachable
            Err(err) => Err(err),
        }
    })
}

// Public eval function that can be called from other modules
pub fn eval(env: &mut Lenv, ast: Box<Lval>) -> ReplispResult<Box<Lval>> {
    let mut ast_mut = *ast;
    lval_eval(env, &mut ast_mut)
}

pub fn lval_eval(env: &mut Lenv, ast: &mut Lval) -> ReplispResult<Box<Lval>> {
    match ast {
        Lval::Sym(s) => {
            // resolve symbol from the env
            let r = env.get(s.as_str())?;

            return Ok(r);
        }
        Lval::Sexpr(cells) => {
            if cells.len() == 0 {
                return Ok(sexpr());
            }

            if cells.len() == 1 {
                return lval_eval(env, &mut *pop(ast, 0)?);
            }

            let mut r: Box<Lval> = eval_cells(env, cells)?;

            let fp = pop(&mut r, 0)?;
            println!("Calling function {:?} on {:?}", fp, ast);
            lval_call(env, *fp, &mut r)
        }
        _ => Ok(Box::new(ast.clone())),
    }
}
/// Execute a sequence of expressions and return the result of the last one
pub fn builtin_do(env: &mut Lenv, v: &mut Lval) -> ReplispResult<Box<Lval>> {
    // Ensure v is an S-expression
    if let Lval::Sexpr(ref mut cells) = *v {
        if cells.is_empty() {
            return Ok(sexpr());
        }

        let mut result = sexpr();

        // Evaluate each expression in sequence
        for i in 0..cells.len() {
            // We need to clone each cell before evaluating it
            let mut cell_clone = (*cells[i]).clone();
            result = lval_eval(env, &mut cell_clone)?;
        }

        Ok(result)
    } else {
        Err(Error::WrongType("sexpr".to_string(), format!("{:?}", v)))
    }
}

pub fn register_builtins(env: &mut Lenv) {
    // Register builtins
    // The "stub" fns are dispatched separately - the function pointer stored is never called
    // these are the ones the modify the environment

    // Definiton
    register_builtin(env, "\\", builtin_lambda);
    register_builtin(env, "fun", builtin_lambda);
    // register_builtin(env, "def", builtin_put_stub);
    // register_builtin(env, "=", builtin_put_stub); // BROKEN

    // List manipulation
    register_builtin(env, "cons", builtin_cons);
    // register_builtin(env, "eval", builtin_eval_stub);
    register_builtin(env, "head", builtin_head);
    register_builtin(env, "init", builtin_init);
    register_builtin(env, "list", builtin_list);
    register_builtin(env, "join", builtin_join);
    register_builtin(env, "len", builtin_len);
    register_builtin(env, "tail", builtin_tail);

    // Utility
    register_builtin(env, "exit", builtin_exit);
    register_builtin(env, "print", builtin_print);
    // register_builtin(env, "printenv", builtin_printenv_stub);

    // Control flow
    // Create a wrapper function for builtin_do that matches the LBuiltin signature
    let do_wrapper: LBuiltin = |_v: &mut Lval| {
        // This is just a stub that will be replaced by special handling in lval_call
        Ok(sexpr())
    };
    register_builtin(env, "do", do_wrapper);

    // Arithmetic
    register_builtin(env, "+", builtin_add);
    register_builtin(env, "add", builtin_add);
    register_builtin(env, "-", builtin_sub);
    register_builtin(env, "sub", builtin_sub);
    register_builtin(env, "*", builtin_mul);
    register_builtin(env, "mul", builtin_mul);
    register_builtin(env, "/", builtin_div);
    register_builtin(env, "div", builtin_div);
    register_builtin(env, "^", builtin_pow);
    register_builtin(env, "pow", builtin_pow);
    register_builtin(env, "%", builtin_rem);
    register_builtin(env, "rem", builtin_rem);
    register_builtin(env, "min", builtin_min);
    register_builtin(env, "max", builtin_max);
}

// register a function pointer to the global scope
fn register_builtin(env: &mut Lenv, name: &str, func: LBuiltin) {
    env.put(name.to_string(), builtin(func, name));
}
