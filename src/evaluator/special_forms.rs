use crate::parser::{Expr, ExprType};
use crate::value::Value;
use crate::error::{EvalError, StackFrame};
use crate::environment::Environment;
use crate::tokenizer::Position;
use crate::evaluator::eval_expr_with_stack;

pub fn eval_def(args: &[Expr], env: &mut Environment, stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    if args.len() != 2 {
        let pos = if args.is_empty() { 
            Position::new(1, 1) // fallback position
        } else { 
            args[0].position.clone() 
        };
        return Err(EvalError::ArityError("def requires exactly 2 arguments".to_string(), pos));
    }
    
    let name = match &args[0].expr_type {
        ExprType::Symbol(s) => s.clone(),
        _ => return Err(EvalError::TypeError("def requires a symbol as first argument".to_string(), args[0].position.clone())),
    };
    
    let value = eval_expr_with_stack(&args[1], env, stack)?;
    env.define(&name, value.clone());
    Ok(value)
}

pub fn eval_defn(args: &[Expr], env: &mut Environment, _stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    if args.len() < 3 {
        let pos = if args.is_empty() { 
            Position::new(1, 1) // fallback position
        } else { 
            args[0].position.clone() 
        };
        return Err(EvalError::ArityError("defn requires at least 3 arguments".to_string(), pos));
    }
    
    let name = match &args[0].expr_type {
        ExprType::Symbol(s) => s.clone(),
        _ => return Err(EvalError::TypeError("defn requires a symbol as first argument".to_string(), args[0].position.clone())),
    };
    
    let params = match &args[1].expr_type {
        ExprType::List(param_exprs) => {
            let mut params = Vec::new();
            for param_expr in param_exprs {
                match &param_expr.expr_type {
                    ExprType::Symbol(s) => params.push(s.clone()),
                    _ => return Err(EvalError::TypeError("defn parameters must be symbols".to_string(), param_expr.position.clone())),
                }
            }
            params
        }
        _ => return Err(EvalError::TypeError("defn requires a parameter list as second argument".to_string(), args[1].position.clone())),
    };
    
    let body = args[2..].to_vec();
    
    let lambda = Value::Lambda {
        params,
        body,
        closure: env.clone(),
    };
    
    // Define function in environment
    env.define(&name, lambda.clone());
    Ok(lambda)
}

pub fn eval_lambda(args: &[Expr], env: &mut Environment, _stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    if args.len() < 2 {
        let pos = if args.is_empty() { 
            Position::new(1, 1)
        } else { 
            args[0].position.clone() 
        };
        return Err(EvalError::ArityError("lambda requires at least 2 arguments".to_string(), pos));
    }
    
    let params = match &args[0].expr_type {
        ExprType::List(param_exprs) => {
            let mut params = Vec::new();
            for param_expr in param_exprs {
                match &param_expr.expr_type {
                    ExprType::Symbol(s) => params.push(s.clone()),
                    _ => return Err(EvalError::TypeError("lambda parameters must be symbols".to_string(), param_expr.position.clone())),
                }
            }
            params
        }
        _ => return Err(EvalError::TypeError("lambda requires a parameter list as first argument".to_string(), args[0].position.clone())),
    };
    
    let body = args[1..].to_vec();
    
    Ok(Value::Lambda {
        params,
        body,
        closure: env.clone(),
    })
}

pub fn eval_do(args: &[Expr], env: &mut Environment, stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    let mut result = Value::Nil;
    for expr in args {
        result = eval_expr_with_stack(expr, env, stack)?;
    }
    Ok(result)
}

pub fn eval_if(args: &[Expr], env: &mut Environment, stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    if args.len() < 2 || args.len() > 3 {
        let pos = if args.is_empty() { 
            Position::new(1, 1)
        } else { 
            args[0].position.clone() 
        };
        return Err(EvalError::ArityError("if requires 2 or 3 arguments (condition, then, optional else)".to_string(), pos));
    }
    
    let condition = eval_expr_with_stack(&args[0], env, stack)?;
    
    if is_truthy(&condition) {
        // Evaluate then branch
        eval_expr_with_stack(&args[1], env, stack)
    } else if args.len() == 3 {
        // Evaluate else branch
        eval_expr_with_stack(&args[2], env, stack)
    } else {
        // No else branch, return nil
        Ok(Value::Nil)
    }
}

pub fn eval_cond(args: &[Expr], env: &mut Environment, stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    for clause in args {
        match &clause.expr_type {
            ExprType::List(clause_elements) => {
                if clause_elements.len() < 2 {
                    return Err(EvalError::TypeError("cond clause must have at least 2 elements (condition and result)".to_string(), clause.position.clone()));
                }
                
                let condition_expr = &clause_elements[0];
                let result_exprs = &clause_elements[1..];
                
                // Check for 'else' clause (special symbol that's always true)
                let is_else_clause = matches!(&condition_expr.expr_type, ExprType::Symbol(s) if s == "else");
                
                let condition_result = if is_else_clause {
                    Value::Boolean(true) // else is always true
                } else {
                    eval_expr_with_stack(condition_expr, env, stack)?
                };
                
                if is_truthy(&condition_result) {
                    // Execute all expressions in the clause, return the last result
                    let mut result = Value::Nil;
                    for expr in result_exprs {
                        result = eval_expr_with_stack(expr, env, stack)?;
                    }
                    return Ok(result);
                }
            }
            _ => {
                return Err(EvalError::TypeError("cond clauses must be lists".to_string(), clause.position.clone()));
            }
        }
    }
    
    // No clause matched, return nil
    Ok(Value::Nil)
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