
use crate::errors::CustomError;
use std::collections::HashMap;
use crate::value::Value;


pub trait Valuable {
    fn get_value(&self, variables: &HashMap<String, Value>) -> Result<Value, CustomError>;
}

pub struct FloatValue {
    value: f64,
}

impl FloatValue {
    pub fn new(value: f64) -> FloatValue {
        FloatValue {value}
    }
}

pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn new(value: &str) -> StringValue {
        StringValue {value: value.to_string()}
    }
}

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        Variable{name: name.to_string()}
    }
}


impl Valuable for FloatValue {
    fn get_value(&self, _variables: &HashMap<String, Value>) -> Result<Value, CustomError> {
        Ok(Value::new_float(self.value))
    }
}

impl Valuable for StringValue {
    fn get_value(&self, _variables: &HashMap<String, Value>) -> Result<Value, CustomError> {
        Ok(Value::new_string(&self.value))
    }
}


impl Valuable for Variable {
    fn get_value(&self, variables: &HashMap<String, Value>) -> Result<Value, CustomError>  {
        match variables.get(&self.name) {
            Some(value) => Ok(value.clone()),
            None => Err(CustomError::new_variable_not_found_error(&self.name)),
        }
    }

}



pub struct Operation {
    left: Box<dyn Valuable>,
    right: Box<dyn Valuable>,
    operator: char,
}

impl Operation {
    pub fn new(left: Box<dyn Valuable>, right: Box<dyn Valuable>, operator: char) -> Operation {
        Operation{left, right, operator}
    }
}

impl Valuable for Operation {
    fn get_value(&self, variables: &HashMap<String, Value>) -> Result<Value, CustomError>  {
        let left; let right;
        match self.left.get_value(variables) {
            Ok(value) => left = value,
            Err(e) => return Err(e),
        }
        match self.right.get_value(variables) {
            Ok(value) => right = value,
            Err(e) => return Err(e),
        }
        match self.operator {
            '+' => Ok(left.add(&right)),
            '-' => Ok(left.sub(&right)),
            '*' => Ok(left.mul(&right)),
            '/' => Ok(left.div(&right)),
            _ => Err(CustomError::new_operator_not_found_error(self.operator)),
        }
    }
}

pub struct Affectation {
    variable: String,
    value: Box<dyn Valuable>,
}

impl Affectation {
    pub fn new(variable: &str, value: Box<dyn Valuable>) -> Affectation {
        Affectation{variable: variable.to_string(), value}
    }
}

pub trait Instruction {
    fn execute(&self, variables: &mut HashMap<String, Value>) -> Result<Value,CustomError>;
}

impl Instruction for Affectation {
    fn execute(&self, variables: &mut HashMap<String, Value>) -> Result<Value,CustomError> {
        let value;
        match self.value.get_value(variables) {
            Ok(v) => value = v,
            Err(e) => return Err(e),
        }

        variables.insert(self.variable.clone(), value);

        //Return None Value
        Ok(Value::String("None".to_string()))
    }
}

impl Instruction for Operation {
    fn execute(&self, variables: &mut HashMap<String, Value>) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl Instruction for Variable {
    fn execute(&self, variables: &mut HashMap<String, Value>) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

pub struct Interpreter {
    pub variables:HashMap<String, Value>,
}

impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter{
            variables:HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn execute (&mut self, instruction: &dyn Instruction) -> Result<Value,CustomError>{
        let result = instruction.execute(&mut self.variables);
        return result;
    }

}