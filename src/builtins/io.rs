use crate::environment::Environment;
use crate::value::Value;
use crate::error::EvalError;

pub fn register(env: &mut Environment) {
    env.define("print", Value::Function(builtin_print));
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