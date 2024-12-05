use std::fmt::Debug;
use crate::errors::CustomError;

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null(),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "Integer({})", i),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::String(s) => write!(f, "String({})", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Null() => write!(f, "Null"),
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

    pub fn new_boolean(b: bool) -> Value {
        Value::Boolean(b)
    }

    pub fn new_null() -> Value {
        Value::Null()
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

    pub fn eq(&self, other: &Value) -> Result<bool, CustomError> {
        let result ;
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => result = a == b,
            (Value::Float(a), Value::Float(b)) => result = a == b,
            (Value::String(a), Value::String(b)) => result = a == b,
            (Value::Boolean(a), Value::Boolean(b)) => result = a == b,
            _ => result = false,
        }
        Ok(result)
    }

    pub fn neq(&self, other: &Value) -> Result<bool, CustomError> {
        match self.eq(other) {
            Ok(eq) => Ok(!eq),
            Err(e) => Err(e),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<bool, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
            (Value::Float(a), Value::Float(b)) => Ok(a > b),
            _ => Err(CustomError::new_operation_error(format!("Cannot compare {:?} and {:?}", self, other).as_str())),
        }
    }

    pub fn lt(&self, other: &Value) -> Result<bool, CustomError> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
            (Value::Float(a), Value::Float(b)) => Ok(a < b),
            _ => Err(CustomError::new_operation_error(format!("Cannot compare {:?} and {:?}", self, other).as_str())),
        }
    }


    pub fn clone(&self) -> Value {
        match self {
            Value::Integer(i) => Value::Integer(*i),
            Value::Float(f) => Value::Float(*f),
            Value::String(s) => Value::String(s.clone()),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::Null() => Value::Null(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null() => "None".to_string(),
        }
    }

}