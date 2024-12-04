use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

pub struct CustomError {
    message: String,
    error_type: String,
}


impl CustomError {
    pub fn new(message: &str, error_type: &str) -> CustomError {
        CustomError{message: message.to_string(), error_type: error_type.to_string()}
    }

    pub fn new_variable_not_found_error(variable_name: &str) -> CustomError {
        CustomError::new(&format!("Variable {} does not exist", variable_name), "VariableNotFoundError")
    }

    pub fn new_operator_not_found_error(operator: char) -> CustomError {
        CustomError::new(&format!("Operator {} not found", operator), "OperatorNotFoundError")
    }

    pub fn new_lexer_error(message: &str) -> CustomError {CustomError::new(message, "LexerError")
    }

    pub fn new_parser_error(message: &str) -> CustomError{CustomError::new(message, "ParserError")}

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_error_type(&self) -> &str {
        &self.error_type
    }

    pub fn equals(&self, error: &CustomError) -> bool {
        self.message == error.message && self.error_type == error.error_type
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

