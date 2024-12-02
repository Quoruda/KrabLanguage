use std::fmt::Debug;

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "Integer({})", i),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::String(s) => write!(f, "String({})", s),
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

    pub fn add(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::String(a), Value::String(b)) => Value::String(format!("{}{}", a, b)),
            _ => panic!("Cannot add {:?} and {:?}", self, other),
        }
    }

    pub fn sub(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            _ => panic!("Cannot subtract {:?} and {:?}", self, other),
        }
    }

    pub fn mul(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            _ => panic!("Cannot multiply {:?} and {:?}", self, other),
        }
    }

    pub fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            _ => panic!("Cannot divide {:?} and {:?}", self, other),
        }
    }

    pub fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }

    pub fn neq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a != b,
            (Value::Float(a), Value::Float(b)) => a != b,
            (Value::String(a), Value::String(b)) => a != b,
            _ => false,
        }
    }

    pub fn clone(&self) -> Value {
        match self {
            Value::Integer(i) => Value::Integer(*i),
            Value::Float(f) => Value::Float(*f),
            Value::String(s) => Value::String(s.clone()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
        }
    }

}