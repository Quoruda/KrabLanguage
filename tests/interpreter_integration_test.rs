extern crate KrabLanguage;
use KrabLanguage::interpreter::{StringValue, Interpreter, FloatValue, Variable, Affectation, Operation, IntegerValue};
use KrabLanguage::value::Value;
use KrabLanguage::errors::CustomError;
use KrabLanguage::value::Value::Integer;

fn get_interpreter() -> Interpreter {
    Interpreter::new()
}

#[test]
fn test_affectation_number() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let _ = interpreter.execute(&affectation);
    let result = interpreter.get_variable("a");
    match result {
        Some(value) => assert!(value.eq(&Value::new_float(20.0))),
        None => assert!(false)
    }
}

#[test]
fn test_affectation_string() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(StringValue::new("Hello")));
    let _ = interpreter.execute(&affectation);
    let result = interpreter.get_variable("a");
    match result {
        Some(value) => assert!(value.eq(&Value::new_string("Hello"))),
        None =>  assert!(false)
    }
}

#[test]
fn test_operation_number() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(FloatValue::new(20.0)), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_float(40.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_operation_string() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(StringValue::new("World")), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_string("HelloWorld"))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_operation_with_variable() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let _ = interpreter.execute(&affectation);
    let operation = Operation::new(Box::new(Variable::new("a")), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_float(40.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_multiplication_between_string_and_integer() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(IntegerValue::new(2)), '*');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_string("HelloHello"))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_multiple_operations() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let _ = interpreter.execute(&affectation);
    let operation = Operation::new(Box::new(Variable::new("a")), Box::new(FloatValue::new(20.0)), '+');
    let operation = Operation::new(Box::new(operation), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_float(60.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_all_operations_number() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '+');
    let operation = Operation::new(Box::new(operation), Box::new(FloatValue::new(20.0)), '-');
    let operation = Operation::new(Box::new(operation), Box::new(FloatValue::new(20.0)), '*');
    let operation = Operation::new(Box::new(operation), Box::new(FloatValue::new(20.0)), '/');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_float(10.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_get_number_with_execute(){
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_float(30.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_get_string_with_execute(){
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(StringValue::new("World")), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(value.eq(&Value::new_string("HelloWorld"))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_non_existing_variable(){
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(Variable::new("b")));
    let result = interpreter.execute(&affectation);
    match result {
        Ok(_) => assert!(false),
        Err(e) => assert!(e.equals(&CustomError::new_variable_not_found_error("b")))
    }
}

