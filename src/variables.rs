use std::collections::HashMap;
use crate::errors::CustomError;
use crate::value::Value;

pub struct VariableManager {
    variables: HashMap<String, Value>,
}

impl VariableManager {
    pub fn new() -> VariableManager {
        VariableManager {variables: HashMap::new()}
    }

    pub fn get_variable(&self, name: &str) -> Result<Value, CustomError> {
        match self.variables.get(name){
            Some(value) => return Ok(value.clone()),
            None => return Err(CustomError::new_variable_not_found_error(name)),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
}