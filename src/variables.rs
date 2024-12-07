use std::collections::HashMap;
use crate::errors::CustomError;
use crate::value::Value;

pub struct VariableManager {
    variables: Vec<HashMap<String, Value>>,
    scope:usize,
}



impl VariableManager {
    pub fn new() -> VariableManager {
        VariableManager {variables: vec![HashMap::new()]
        , scope:1}
    }

    pub fn get_variable(&self, name: &str) -> Result<Value, CustomError> {
        for i in 0..self.scope{
            match self.variables[i].get(name){
                Some(value) => return Ok(value.clone()),
                None => continue,
            }
        }
        return Err(CustomError::new_variable_not_found_error(name))
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        for i in 0..self.scope{
            match self.variables[i].get(name){
                Some(_) => {
                    self.variables[i].insert(name.to_string(), value);
                    return;
                },
                None => continue,
            }
        }
        self.variables[self.scope-1].insert(name.to_string(), value);
    }

    pub fn enter_scope(&mut self){
        self.scope += 1;
        self.variables.push(HashMap::new());
    }

    pub fn exit_scope(&mut self){
        self.variables.pop();
        self.scope -= 1;
    }
}