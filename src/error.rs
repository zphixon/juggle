
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    SyntaxError,
    HandsUnderflowError,
    AirUnderflowError,
    TypeError,
    MismatchingEndError,
}

#[derive(Debug)]
pub struct Error {
    pub which: ErrorType,
    pub message: String,
    pub line: u64,
}

impl Error {
    pub fn new(which: ErrorType, message: String, line: u64) -> Error {
        Error {
            which: which,
            message: message,
            line: line,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has occurred: {:?}\n    On line {}: {}", self.which, self.line, self.message)
    }
}

