use crate::object::Object;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum SantaError {
    #[fail(display = "Parser error: {}", cause)]
    ParseError { cause: String },

    #[fail(display = "Error in parse tree construction: {}", cause)]
    ParseTreeError { cause: String },

    #[fail(display = "Operation not supported: {}", cause)]
    InvalidOperationError { cause: String },

    #[fail(display = "Index out of bounds")]
    IndexOutOfBounds,

    #[fail(display = "KeyError, key not found")]
    KeyError,

    #[fail(
        display = "This exception is raised when a fucntion wants to return.\
The evaluator will never actually raise this error but will instead return it's value.\
value: {}",
        value
    )]
    ReturnException { value: Object },
}
