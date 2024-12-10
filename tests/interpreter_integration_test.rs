extern crate KrabLanguage;
use KrabLanguage::interpreter::{StringValue, Interpreter, FloatValue, Variable, Affectation, Operation, IntegerValue, Condition, ConditionBlock, ConditionLoop, Instruction, InstructionBlock};
use KrabLanguage::value::Value;
use KrabLanguage::errors::CustomError;

fn get_interpreter() -> Interpreter {
    Interpreter::new()
}

fn eq_values(value1: &Value, value2: &Value) -> bool {
    match value1.eq(value2) {
        Ok(b) => b,
        Err(_) => false
    }
}

#[test]
fn test_affectation_number() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let _ = interpreter.execute(&affectation);
    let result = interpreter.get_variable("a");
    match result {
        Ok(value) => assert!(eq_values(&value,&Value::new_float(20.0))),
        Err(_) =>  assert!(false)
    }
}

#[test]
fn test_affectation_string() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(StringValue::new("Hello")));
    let _ = interpreter.execute(&affectation);
    let result = interpreter.get_variable("a");
    match result {
        Ok(value) => assert!(eq_values(&value,&Value::new_string("Hello"))),
        Err(_) =>  assert!(false)
    }
}

#[test]
fn test_operation_number() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(FloatValue::new(20.0)), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_float(40.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_operation_string() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(StringValue::new("World")), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_string("HelloWorld"))),
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
        Ok(value) => assert!(eq_values(&value, &Value::new_float(40.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_multiplication_between_string_and_integer() {
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(IntegerValue::new(2)), '*');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_string("HelloHello"))),
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
        Ok(value) => assert!(eq_values(&value, &Value::new_float(60.0))),
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
        Ok(value) => assert!(eq_values(&value, &Value::new_float(10.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_get_number_with_execute(){
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_float(30.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_get_string_with_execute(){
    let mut interpreter = get_interpreter();
    let operation = Operation::new(Box::new(StringValue::new("Hello")), Box::new(StringValue::new("World")), '+');
    let result = interpreter.execute(&operation);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_string("HelloWorld"))),
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

#[test]
fn test_instruction_block(){
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(10.0)));
    let _ = interpreter.execute(&affectation);
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let affectation2 = Affectation::new("b", Box::new(FloatValue::new(30.0)));
    let instruction_block = InstructionBlock::new(vec![Box::new(affectation), Box::new(affectation2)]);
    let result = interpreter.execute(&instruction_block);
    match result {
        Ok(_) => {},
        Err(_) => assert!(false)
    }
    let var = interpreter.get_variable("a");
    match var {
        Ok(value) => assert!(eq_values(&value, &Value::new_float(20.0))),
        Err(_) => assert!(false)
    }
    let var = interpreter.get_variable("b");
    match var {
        Ok(_) => assert!(false),
        Err(e) => assert!(e.equals(&CustomError::new_variable_not_found_error("b")))
    }
}

#[test]
fn test_condition_less_than(){
    let mut interpreter = get_interpreter();
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '<');
    let result = interpreter.execute(&condition);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_boolean(true))),
        Err(_) => assert!(false)
    }
    let condition = Condition::new(Box::new(FloatValue::new(20.0)), Box::new(FloatValue::new(10.0)), '<');
    let result = interpreter.execute(&condition);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_boolean(false))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_condition_greater_than(){
    let mut interpreter = get_interpreter();
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '>');
    let result = interpreter.execute(&condition);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_boolean(false))),
        Err(_) => assert!(false)
    }
    let condition = Condition::new(Box::new(FloatValue::new(20.0)), Box::new(FloatValue::new(10.0)), '>');
    let result = interpreter.execute(&condition);
    match result {
        Ok(value) => assert!(eq_values(&value, &Value::new_boolean(true))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_error_condition(){
    let mut interpreter = get_interpreter();
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), 'a');
    let result = interpreter.execute(&condition);
    match result {
        Ok(_) => assert!(false),
        Err(e) => assert!(e.equals(&CustomError::new_operator_not_found_error('a')))
    }
}

#[test]
fn test_condition_block_success(){
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(0.0)));
    let _ = interpreter.execute(&affectation);
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '<');
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let condition_block = ConditionBlock::new(Box::new(condition),  InstructionBlock::new(vec![Box::new(affectation)]));
    let _result = interpreter.execute(&condition_block);
    let var = interpreter.get_variable("a");
    match var {
        Ok(value) => assert!(eq_values(&value, &Value::new_float(20.0))),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_access_variable_in_condition_block(){
    let mut interpreter = get_interpreter();
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '<');
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let condition_block = ConditionBlock::new(Box::new(condition), InstructionBlock::new(vec![Box::new(affectation)]));
    let _result = interpreter.execute(&condition_block);
    let var = interpreter.get_variable("a");
    match var {
        Ok(_) => assert!(false),
        Err(e) => assert!(e.equals(&CustomError::new_variable_not_found_error("a")))
    }
}

#[test]
fn test_condition_block_fail() {
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(FloatValue::new(10.0)));
    let _ = interpreter.execute(&affectation);
    let condition = Condition::new(Box::new(FloatValue::new(10.0)), Box::new(FloatValue::new(20.0)), '>');
    let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
    let condition_block = ConditionBlock::new(Box::new(condition),  InstructionBlock::new(vec![Box::new(affectation)]));
    let _result = interpreter.execute(&condition_block);
    let var = interpreter.get_variable("a");
    match var {
        Ok(_) => assert!(eq_values(&var.unwrap(), &Value::new_float(10.0))),
        Err(_) => assert!(true)
    }
}

#[test]
fn test_condition_loop(){
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let _ = interpreter.execute(&affectation);

    let condition = Condition::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(100)), '<');
    let operation = Operation::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(1)), '+');
    let affectation = Affectation::new("a", Box::new(operation));
    let condition_loop = ConditionLoop::new(Box::new(condition),   InstructionBlock::new( vec![Box::new(affectation)]));
    let result = interpreter.execute(&condition_loop);
    match result {
        Ok(_) => {},
        Err(_) => assert!(false)
    }
    match interpreter.get_variable("a"){
        Ok(v) => assert!(eq_values(&v,&Value::new_integer(100) )),
        Err(_) => assert!(false)
    }
}
#[test]
fn test_access_variable_in_condition_loop(){
    let mut interpreter = get_interpreter();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let _ = interpreter.execute(&affectation);
    let condition = Condition::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(1)), '<');
    let operation = Operation::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(1)), '+');
    let affectation = Affectation::new("a", Box::new(operation));
    let affectation2 = Affectation::new("b", Box::new(IntegerValue::new(1)));
    let condition_loop = ConditionLoop::new(Box::new(condition),   InstructionBlock::new(vec![Box::new(affectation), Box::new(affectation2)]));
    let result = interpreter.execute(&condition_loop);
    match result {
        Ok(_) => {},
        Err(_) => assert!(false)
    }
    match interpreter.get_variable("b"){
        Ok(_) => assert!(false),
        Err(e) => assert!(e.equals(&CustomError::new_variable_not_found_error("b")))
    }
}