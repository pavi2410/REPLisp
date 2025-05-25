use chumsky::error::Simple;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Division by zero")]
    DivideByZero,

    #[error("Empty list")]
    EmptyList,

    #[error("No children in expression")]
    NoChildren,

    #[error("Not a number")]
    NotANumber,

    #[error("Expected {0} arguments, got {1}")]
    NumArguments(usize, usize),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Expected {0}, got {1}")]
    WrongType(String, String),

    #[error("Unknown function: {0}")]
    UnknownFunction(String),
}

pub type ReplispResult<T> = Result<T, Error>;

impl<T: Display + Hash + Eq + std::fmt::Debug> From<Simple<T>> for Error {
    fn from(error: Simple<T>) -> Self {
        Error::Parse(format!("{:?}", error))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        Error::NotANumber
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Parse(error.to_string())
    }
}
