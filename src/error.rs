use std::fmt::Debug;
use std::hash::Hash;


#[derive(Debug)]
pub enum Error {
    DivideByZero,
    EmptyList,
    FunctionFormat,
    NoChildren,
    NotANumber,
    NumArguments(usize, usize),
    Parse(String),
    WrongType(String, String),
    UnknownFunction(String),
}

pub type ReplispResult<T> = Result<T, Error>;

impl<T> From<pest::error::Error<T>> for Error
where
	T: Debug + Ord + Copy + Hash,
{
	fn from(error: pest::error::Error<T>) -> Self {
		Error::Parse(format!("{error}"))
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