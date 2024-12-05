use std::fmt::Debug;
use crate::errors::CustomError;

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "Integer({})", i),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::String(s) => write!(f, "String({})", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
        }
    }
}

impl Value {

    pub fn new_string(s: &str) -> Value {
        Value::String(s.to_string())
    }

    pub fn new_integer(i: i64) -> Value {
        Value::Integer(i)
    }

    pub fn new_float(f: f64) -> Value {
        Value::Float(f)
    }

    pub fn add(&self, other: &Value) -> Result<Value, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => Err(CustomError::new_operation_error(format!("Cannot add {:?} and {:?}", self, other).as_str())),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(CustomError::new_operation_error(format!("Cannot subtract {:?} and {:?}", self, other).as_str())),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            (Value::String(a), Value::Integer(b)) => Ok(Value::String(a.repeat(*b as usize))),
            _ => Err(CustomError::new_operation_error(format!("Cannot multiply {:?} and {:?}", self, other).as_str())),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            _ => Err(CustomError::new_operation_error(format!("Cannot divide {:?} and {:?}", self, other).as_str())),
        }
    }

    pub fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            _ => false,
        }
    }


    pub fn clone(&self) -> Value {
        match self {
            Value::Integer(i) => Value::Integer(*i),
            Value::Float(f) => Value::Float(*f),
            Value::String(s) => Value::String(s.clone()),
            Value::Boolean(b) => Value::Boolean(*b),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
        }
    }

}