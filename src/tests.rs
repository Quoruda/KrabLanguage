
#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::interpreter::Affectation;
    use crate::interpreter::Number;
    use crate::interpreter::Operation;
    use crate::interpreter::Variable;

    fn get_interpreter() -> Interpreter {
        Interpreter::new_for_tests()
    }

    #[test]
    fn test_affection() {
        let mut interpreter = get_interpreter();
        let instruction = Affectation::new("a", Box::new(Number::new(10.0)));
        interpreter.execute(&instruction);
        assert_eq!(interpreter.get_variable("a"), Some(&10.0));
    }

    #[test]
    fn test_operation() {
        let mut interpreter = get_interpreter();
        let operation = Operation::new(Box::new(Number::new(10.0)), Box::new(Number::new(20.0)), '+');
        let instruction = Affectation::new("c", Box::new(operation));
        interpreter.execute(&instruction);
        assert_eq!(interpreter.get_variable("c"), Some(&30.0));
    }

    #[test]
    fn test_operation_with_variable() {
        let mut interpreter = get_interpreter();
        let instruction = Affectation::new("a", Box::new(Number::new(10.0)));
        interpreter.execute(&instruction);
        let operation = Operation::new(Box::new(Variable::new("a")), Box::new(Number::new(20.0)), '+');
        let instruction = Affectation::new("c", Box::new(operation));
        interpreter.execute(&instruction);
        assert_eq!(interpreter.get_variable("c"), Some(&30.0));
    }

    #[test]
    fn test_multiple_operations() {
        let mut interpreter = get_interpreter();
        let number1 = Number::new(10.0);
        let number2 = Number::new(20.0);
        let number3 = Number::new(36.0);
        let operation1 = Operation::new(Box::new(number1), Box::new(number2), '+');
        let operation2 = Operation::new(Box::new(operation1), Box::new(number3), '+');
        let instruction = Affectation::new("c", Box::new(operation2));
        interpreter.execute(&instruction);
        assert_eq!(interpreter.get_variable("c"), Some(&66.0));
    }

    #[test]
    fn test_all_operations() {
        let mut interpreter = get_interpreter();

        let addition = Operation::new(Box::new(Number::new(10.0)), Box::new(Number::new(20.0)), '+');
        let addition_instruction = Affectation::new("a", Box::new(addition));
        interpreter.execute(&addition_instruction);
        assert_eq!(interpreter.get_variable("a"), Some(&30.0));

        let subtraction = Operation::new(Box::new(Number::new(10.0)), Box::new(Number::new(20.0)), '-');
        let subtraction_instruction = Affectation::new("s", Box::new(subtraction));
        interpreter.execute(&subtraction_instruction);
        assert_eq!(interpreter.get_variable("s"), Some(&-10.0));

        let multiplication = Operation::new(Box::new(Number::new(10.0)), Box::new(Number::new(20.0)), '*');
        let multiplication_instruction = Affectation::new("m", Box::new(multiplication));
        interpreter.execute(&multiplication_instruction);
        assert_eq!(interpreter.get_variable("m"), Some(&200.0));

        let division = Operation::new(Box::new(Number::new(10.0)), Box::new(Number::new(20.0)), '/');
        let division_instruction = Affectation::new("d", Box::new(division));
        interpreter.execute(&division_instruction);
        assert_eq!(interpreter.get_variable("d"), Some(&0.5));
    }

    #[test]
    fn test_get_value_with_execute(){
        let mut interpreter = get_interpreter();
        let number1 = Number::new(10.0);
        let number2 = Number::new(20.0);
        let operation = Operation::new(Box::new(number1), Box::new(number2), '+');
        match interpreter.execute(&operation){
            Ok(result) => assert_eq!(result.get_value(&interpreter.variables), 30.0),
            Err(_) => panic!("Error")
        }
        let instruction = Affectation::new("a", Box::new(operation));
        match interpreter.execute(&instruction){
            Ok(result) => assert_eq!(result.get_value(&interpreter.variables), 0.0),
            Err(_) => panic!("Error")
        }
    }
    /*

    #[test]
    fn test_error_non_existing_variable(){
        let mut interpreter = get_interpreter();
        let operation = Operation::new(Box::new(Variable::new("a")), Box::new(Number::new(20.0)), '+');
        match interpreter.execute(&operation){
            Ok(_) => panic!("Error"),
            Err(_) => assert!(true)
        }
    }
    
     */
}