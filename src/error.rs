use failure::Fail;

#[derive(Debug, Fail)]
pub enum SantaError {
    #[fail(display = "Parser error: {}", cause)]
    ParseError { cause: String },

    #[fail(display = "Error in parse tree construction: {}", cause)]
    ParseTreeError { cause: String },

    #[fail(display = "Operation not supported: {}", cause)]
    InvalidOperationError { cause: String },
}
