use std::ops::Deref;
use crate::errors::CustomError;
use crate::value::Value;
use crate::variables::VariableManager;


pub trait Valuable {
    fn get_value(&self, variables: &mut VariableManager) -> Result<Value, CustomError>;
}

pub struct FloatValue {
    value: f64,
}

pub struct IntegerValue {
    value: i64,
}

impl FloatValue {
    pub fn new(value: f64) -> FloatValue {
        FloatValue {value}
    }
}

impl IntegerValue {
    pub fn new(value: i64) -> IntegerValue {
        IntegerValue {value}
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
    fn get_value(&self, _variables: &mut VariableManager) -> Result<Value, CustomError> {
        Ok(Value::new_float(self.value))
    }
}

impl Valuable for IntegerValue {
    fn get_value(&self, _variables: &mut VariableManager) -> Result<Value, CustomError> {
        Ok(Value::new_integer(self.value))
    }
}

impl Valuable for StringValue {
    fn get_value(&self, _variables: &mut VariableManager) -> Result<Value, CustomError> {
        Ok(Value::new_string(&self.value))
    }
}


impl Valuable for Variable {
    fn get_value(&self, variables: &mut VariableManager) -> Result<Value, CustomError>  {
        match variables.get_variable(&self.name) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
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
    fn get_value(&self, variables: &mut VariableManager) -> Result<Value, CustomError>  {
        match self.left.get_value(variables) {
            Ok(value) => {
                match self.right.get_value(variables) {
                    Ok(value2) => {
                        match self.operator {
                            '+' => return value.add(&value2),
                            '-' => return value.sub(&value2),
                            '*' => return value.mul(&value2),
                            '/' => return value.div(&value2),
                            _ => return Err(CustomError::new_operator_not_found_error(self.operator)),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
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
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError>;
}

impl Instruction for Affectation {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError> {
        let value;
        match self.value.get_value(variables) {
            Ok(v) => value = v,
            Err(e) => return Err(e),
        }
        variables.set_variable(&self.variable, value);
        Ok(Value::Null())
    }
}

impl Instruction for Operation {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl Instruction for Variable {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

pub struct Condition{
    left: Box<dyn Valuable>,
    right: Box<dyn Valuable>,
    operator: char,
}

impl Instruction for Box<dyn Valuable> {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl Condition {
    pub fn new(left: Box<dyn Valuable>, right: Box<dyn Valuable>, operator: char) -> Condition {
        Condition{left, right, operator}
    }

    pub fn is_true(&self, variables: &mut VariableManager) -> Result<bool, CustomError> {
        let left; let right;
        match self.left.get_value(variables) {
            Ok(value) => left = value,
            Err(e) => return Err(e),
        }
        match self.right.get_value(variables) {
            Ok(value) => right = value,
            Err(e) => return Err(e),
        }
        let result;
        match self.operator {
            '>' => result = left.gt(&right),
            '<' => result = left.lt(&right),
            '=' => result = left.eq(&right),
            '≠' => result = left.neq(&right),
            _ => return Err(CustomError::new_operator_not_found_error(self.operator)),
        }
        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl Valuable for Condition{
    fn get_value(&self, variables: &mut VariableManager) -> Result<Value, CustomError>  {
        match self.is_true(variables) {
            Ok(value) => Ok(Value::new_boolean(value)),
            Err(e) => Err(e),
        }
    }
}

impl Instruction for Condition {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value,CustomError> {
        match self.get_value(variables) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

pub struct Interpreter {
    pub variables:VariableManager,
}

pub struct ConditionBlock {
    conditions: Box<dyn Valuable>,
    instructions: InstructionBlock,
}

impl ConditionBlock {
    pub fn new(conditions: Box<dyn Valuable>, instructions:InstructionBlock) -> ConditionBlock {
        ConditionBlock{conditions, instructions}
    }
}

pub struct InstructionBlock {
    instructions: Vec<Box<dyn Instruction>>,
}

impl InstructionBlock {
    pub fn new(instructions: Vec<Box<dyn Instruction>>) -> InstructionBlock {
        InstructionBlock{instructions}
    }
}

impl Instruction for InstructionBlock {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value, CustomError> {
        variables.enter_scope();
        for instruction in &self.instructions {
            match instruction.execute(variables) {
                Ok(_) => (),
                Err(e) => {
                    variables.exit_scope();
                    return Err(e)
                },
            }
        }
        variables.exit_scope();
        return Ok(Value::Null())
    }
}

impl Instruction for ConditionBlock {
    fn execute(&self, variables: &mut VariableManager) -> Result<Value, CustomError> {
        let condition;
        match self.conditions.get_value(variables) {
            Ok(value) => {
                match value {
                    Value ::Boolean(b) => condition = b,
                    Value::Null() => condition = false,
                    _ => condition = false,
                }
            },
            Err(e) => return Err(e),
        }
        if condition {
            return self.instructions.execute(variables);
        }
        return Ok(Value::Null())
    }
}

pub struct ConditionLoop{
    condition: Box<dyn Valuable>,
    instructions: InstructionBlock,
}

impl ConditionLoop{
    pub fn new(condition: Box<dyn Valuable>, instructions: InstructionBlock) -> ConditionLoop {
        ConditionLoop{condition, instructions}
    }
}

impl Instruction for ConditionLoop{
    fn execute(&self, variables: &mut VariableManager) -> Result<Value, CustomError> {
        let mut run = true;
        let mut condition:bool;
        while run{
            match self.condition.get_value(variables) {
                Ok(value) => {
                    match value {
                        Value ::Boolean(b) => condition = b,
                        Value::Null() => condition = false,
                        _ => condition = false,
                    }
                },
                Err(e) => return Err(e),
            }
            if condition {
               match self.instructions.execute(variables){
                     Ok(_) => (),
                     Err(e) => return Err(e),
               }
            }else{
                run = false;
            }
        }

        return Ok(Value::new_null())
    }
}

impl Interpreter{
    pub fn new() -> Interpreter{
        Interpreter{
            variables:VariableManager::new(),
        }
    }

    pub fn _get_variable(&mut self, name: &str) -> Result<Value, CustomError> {
        self.variables.get_variable(name)
    }

    pub fn execute (&mut self, instruction: &dyn Instruction) -> Result<Value,CustomError>{
        let result = instruction.execute(&mut self.variables);
        return result;
    }

    pub fn execute_instructions(&mut self, instructions: &Vec<Box<dyn Instruction>>) -> Result<Value,CustomError>{
        for instruction in instructions {
            match self.execute(instruction.deref()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(Value::new_null())
    }

}