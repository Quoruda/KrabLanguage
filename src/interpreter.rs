use crate::errors::CustomError;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};


pub trait Valuable {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64;
}

pub(crate) struct Number {
    value: f64,
}

impl Number {
    pub(crate) fn new(value: f64) -> Number {
        Number{value}
    }
}

pub(crate)  struct Variable {
    name: String,
}

impl Variable {
    pub(crate) fn new(name: &str) -> Variable {
        Variable{name: name.to_string()}
    }
}


impl Valuable for Number {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        self.value
    }
}

impl Valuable for Variable {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        let result = variables.get(&self.name);
        match variables.get(&self.name) {
            Some(value) => *value,
            None => panic!("Variable not found"),
        }
    }

}



pub(crate) struct Operation {
    left: Box<dyn Valuable>,
    right: Box<dyn Valuable>,
    operator: char,
}

impl Operation {
    pub(crate) fn new(left: Box<dyn Valuable>, right: Box<dyn Valuable>, operator: char) -> Operation {
        Operation{left, right, operator}
    }
}

impl Valuable for Operation {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        match self.operator {
            '+' => self.left.get_value(variables) + self.right.get_value(variables),
            '-' => self.left.get_value(variables) - self.right.get_value(variables),
            '*' => self.left.get_value(variables) * self.right.get_value(variables),
            '/' => self.left.get_value(variables) / self.right.get_value(variables),
            _ => panic!("Invalid operator"),
        }
    }
}

pub(crate) struct Affectation {
    variable: String,
    value: Box<dyn Valuable>,
}

impl Affectation {
    pub(crate) fn new(variable: &str, value: Box<dyn Valuable>) -> Affectation {
        Affectation{variable: variable.to_string(), value}
    }
}

pub(crate) trait Instruction {
    fn execute(&self, variables: &mut HashMap<String, f64>) -> Result<Box<dyn Valuable>,Box<dyn Error>>;
}

impl Instruction for Affectation {
    fn execute(&self, variables: &mut HashMap<String, f64>) -> Result<Box<dyn Valuable>,Box<dyn Error>> {
        variables.insert(self.variable.clone(), self.value.get_value(variables));

        //Return None Value
        Ok(Box::new(Number::new(0.0)))
    }
}

impl Instruction for Operation {
    fn execute(&self, variables: &mut HashMap<String, f64>) -> Result<Box<dyn Valuable>,Box<dyn Error>> {
        let value = self.get_value(variables);
        Ok(Box::new(Number::new(value)))
    }
}


pub(crate) struct Interpreter {
    running:bool,
    print_errors:bool,
    pub(crate) variables:HashMap<String, f64>,
}

impl Interpreter{
    pub(crate) fn new() -> Interpreter{
        Interpreter{
            running:true,
            print_errors:true,
            variables:HashMap::new(),
        }
    }

    pub(crate) fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }
    pub(crate) fn new_for_tests() -> Interpreter{
        Interpreter{
            running:true,
            print_errors:false,
            variables:HashMap::new(),
        }
    }


    pub(crate) fn execute (&mut self, instruction: &dyn Instruction) -> Result<Box<dyn Valuable>,Box<dyn Error>>{
        let result = instruction.execute(&mut self.variables);
        return result;
    }

}