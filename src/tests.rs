
#[cfg(test)]
mod tests {
    use std::alloc::dealloc;
    use crate::interpreter::{Interpreter, Affectation, Operation, Variable, FloatValue, StringValue};
    use crate::value::Value;
    use crate::value::Value::String;

    fn get_interpreter() -> Interpreter {
        Interpreter::new_for_tests()
    }

    #[test]
    fn test_affectation_number() {
        let mut interpreter = get_interpreter();
        let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
        interpreter.execute(&affectation);
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
        interpreter.execute(&affectation);
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
        interpreter.execute(&affectation);
        let operation = Operation::new(Box::new(Variable::new("a")), Box::new(FloatValue::new(20.0)), '+');
        let result = interpreter.execute(&operation);
        match result {
            Ok(value) => assert!(value.eq(&Value::new_float(40.0))),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_multiple_operations() {
        let mut interpreter = get_interpreter();
        let affectation = Affectation::new("a", Box::new(FloatValue::new(20.0)));
        interpreter.execute(&affectation);
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
            Ok(v) => assert!(false),
            Err(e) => assert!(e.get_message() == "Variable b does not exist")
        }
    }
}