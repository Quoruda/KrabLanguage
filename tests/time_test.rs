extern crate KrabLanguage;
use KrabLanguage::interpreter::{StringValue, Interpreter, FloatValue, Variable, Affectation, Operation, IntegerValue, Condition, ConditionBlock, ConditionLoop, Instruction, Valuable};
use KrabLanguage::value::Value;
use KrabLanguage::errors::CustomError;
use std::time::Instant;

/*
fn test_speed_condition_loop_p1(){
    let mut interpreter = Interpreter::new();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let operation2 = Operation::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(1)), '+');
    let affectation2 = Affectation::new("a", Box::new(operation2));
    let affectation3 = Affectation::new("b", Box::new(Operation::new(Box::new(StringValue::new("a")), Box::new(IntegerValue::new(1000)), '*')));
    let condition = Condition::new(Box::new(Variable::new("a")),Box::new( IntegerValue::new(1000000)), '<');
    let condition_loop = ConditionLoop::new(condition, vec![Box::new(affectation2), Box::new(affectation3)]);

    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    instructions.push(Box::new(affectation));
    instructions.push(Box::new(condition_loop));

    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("With condition loop : {:?}", elapsed.as_secs_f64());
}

fn test_speed_condition_loop_p2(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    instructions.push(Box::new(affectation));
    for i in 0..1000000{
        let condition = Condition::new(Box::new(IntegerValue::new(i)),Box::new( IntegerValue::new(1000000)), '<');
        let affectation = Affectation::new("b", Box::new(Operation::new(Box::new(StringValue::new("a")), Box::new(IntegerValue::new(1000)), '*')));
        let operation2 = Operation::new(Box::new(Variable::new("a")), Box::new(IntegerValue::new(1)), '+');
        let affectation2 = Affectation::new("a", Box::new(operation2));
        instructions.push(Box::new(affectation));
        instructions.push(Box::new(condition));
        instructions.push(Box::new(affectation2));
    }

    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Without condition loop: {:?}", elapsed.as_secs_f64());
}

 */

fn speed_test_operation(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let n = 1_000_000;
    for i in 0..n{
        let operation = Operation::new(Box::new(IntegerValue::new(i)), Box::new(IntegerValue::new(n-i)), '+');
        instructions.push(Box::new(operation));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Operation: {:?}", elapsed.as_secs_f64());
}

fn speed_test_affectation_with_existing_variable(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let _  = interpreter.execute(&affectation);
    let n = 1_000_000;
    for i in 0..n{
        let affectation = Affectation::new("a", Box::new(IntegerValue::new(i)));
        instructions.push(Box::new(affectation));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Affectation with existing variable : {:?}", elapsed.as_secs_f64());
}

fn speed_test_affectation_with_non_existing_variable(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let _  = interpreter.execute(&affectation);
    let n = 1_000_000;
    for i in 0..n{
        let affectation = Affectation::new(&format!("a{}", i), Box::new(IntegerValue::new(i)));
        instructions.push(Box::new(affectation));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Affectation with non existing variable : {:?}", elapsed.as_secs_f64());
}

fn speed_test_get_value_of_variable(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let affectation = Affectation::new("a", Box::new(IntegerValue::new(0)));
    let _  = interpreter.execute(&affectation);
    let n = 1_000_000;
    for i in 0..n{
        let instruction = Variable::new("a");
        instructions.push(Box::new(instruction));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Get value of variable: {:?}", elapsed.as_secs_f64());
}

fn speed_test_get_value_of__basic_value(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let n = 1_000_000;
    for i in 0..n{
        let instruction:Box<dyn Valuable> = Box::new(IntegerValue::new(i));
        instructions.push(Box::new(instruction));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Get value of basic value: {:?}", elapsed.as_secs_f64());
}

fn speed_test_condition(){
    let mut interpreter = Interpreter::new();
    let mut instructions:Vec<Box<dyn Instruction>> = Vec::new();
    let n = 1_000_000;
    for i in 0..n{
        let condition = Condition::new(Box::new(IntegerValue::new(i)),Box::new( IntegerValue::new(n-i)), '<');
        instructions.push(Box::new(condition));
    }
    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Condition: {:?}", elapsed.as_secs_f64());
}

#[test]
fn time_test(){
    speed_test_operation();
    speed_test_affectation_with_existing_variable();
    speed_test_affectation_with_non_existing_variable();
    speed_test_get_value_of__basic_value();
    speed_test_get_value_of_variable();
    speed_test_condition();
}