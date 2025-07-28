pub mod tokenizer;
pub mod parser;
pub mod value;
pub mod error;
pub mod environment;
pub mod evaluator;
pub mod builtins;
pub mod repl;
pub mod file_exec;

// Re-export commonly used types
pub use value::Value;
pub use error::{EvalError, EvalErrorWithStack};
pub use environment::Environment;
pub use evaluator::eval_expr;

#[cfg(test)]
mod test_line_numbers;