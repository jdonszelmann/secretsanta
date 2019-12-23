use crate::object::Object;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum SantaError {
    ParseError { cause: String },
    ParseTreeError { cause: String },
    InvalidOperationError { cause: String },
    IndexOutOfBounds,
    KeyError,
    NoDefinitionError,
    DatabaseError {cause: String},
    ReturnException { value: Object },
    AssertionError,
}

impl Error for SantaError {}

impl Display for SantaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::ReturnException {value} => write!(f, "This exception is raised when a fucntion wants to return. The evaluator will never actually raise this error but will instead return it's value. value: {}", value),
            Self::InvalidOperationError {cause} => write!(f, "Operation not supported: {}", cause),
            Self::ParseTreeError {cause} => write!(f, "Error in parse tree construction: {}", cause),
            Self::ParseError {cause} => write!(f, "Parser error: {}", cause),
            Self::IndexOutOfBounds => write!(f, "Index out of bounds"),
            Self::KeyError => write!(f, "KeyError, key not found"),
            Self::NoDefinitionError => write!(f, "Variable not defined"),
            Self::DatabaseError {cause} => write!(f, "A database error occured: {}", cause),
            Self::AssertionError => write!(f, "Assertion failed"),
        }
    }
}
