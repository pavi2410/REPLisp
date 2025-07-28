use crate::parser::{Expr, ExprType};
use crate::value::Value;
use crate::error::{EvalError, StackFrame};
use crate::environment::Environment;
use crate::evaluator::eval_expr_with_stack;

pub fn eval_function_call(elements: &[Expr], env: &mut Environment, stack: &mut Vec<StackFrame>) -> Result<Value, EvalError> {
    if elements.is_empty() {
        return Ok(Value::List(vec![]));
    }
    
    let func_expr = &elements[0];
    let args_exprs = &elements[1..];
    
    // Get function name for stack trace
    let func_name = match &func_expr.expr_type {
        ExprType::Symbol(name) => name.clone(),
        _ => "<anonymous>".to_string(),
    };
    
    // Evaluate function
    let func = eval_expr_with_stack(func_expr, env, stack)?;
    
    // Evaluate arguments
    let mut args = Vec::new();
    for arg_expr in args_exprs {
        args.push(eval_expr_with_stack(arg_expr, env, stack)?);
    }
    
    // Call function
    match func {
        Value::Function(f) => {
            f(&args).map_err(|mut err| {
                err.set_position(func_expr.position.clone());
                err
            })
        },
        Value::Lambda { params, body, mut closure } => {
            // Check arity
            if args.len() != params.len() {
                return Err(EvalError::ArityError(format!(
                    "Function {} expects {} arguments, got {}",
                    func_name,
                    params.len(),
                    args.len()
                ), func_expr.position.clone()));
            }
            
            // Add to stack trace
            stack.push(StackFrame {
                function_name: func_name,
                position: func_expr.position.clone(),
            });
            
            // Merge current environment into closure for recursive calls
            for (name, value) in &env.bindings {
                if !closure.bindings.contains_key(name) {
                    closure.define(name, value.clone());
                }
            }
            
            // Bind arguments to parameters in closure environment
            for (param, arg) in params.iter().zip(args.iter()) {
                closure.define(param, arg.clone());
            }
            
            // Evaluate body expressions in sequence, return last result
            let mut result = Value::Nil;
            for expr in &body {
                result = eval_expr_with_stack(expr, &mut closure, stack)?;
            }
            
            // Remove from stack trace
            stack.pop();
            Ok(result)
        }
        _ => Err(EvalError::InvalidFunction(format!("Not a function: {:?}", func), func_expr.position.clone())),
    }
}