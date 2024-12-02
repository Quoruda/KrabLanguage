use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

pub(crate) struct CustomError {
    message: String,
    error_type: String,
}


impl CustomError {
    pub fn new(message: &str, error_type: &str) -> CustomError {
        CustomError{message: message.to_string(), error_type: error_type.to_string()}
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl Error for CustomError {}

