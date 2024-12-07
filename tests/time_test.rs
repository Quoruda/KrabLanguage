extern crate KrabLanguage;
use KrabLanguage::interpreter::{StringValue, Interpreter, FloatValue, Variable, Affectation, Operation, IntegerValue, Condition, ConditionBlock, ConditionLoop, Instruction};
use KrabLanguage::value::Value;
use KrabLanguage::errors::CustomError;
use std::time::Instant;

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
    for i in 0..1000000{
        let condition = Condition::new(Box::new(IntegerValue::new(i)),Box::new( IntegerValue::new(1000000)), '<');
        let affectation = Affectation::new("b", Box::new(Operation::new(Box::new(StringValue::new("a")), Box::new(IntegerValue::new(1000)), '*')));
        instructions.push(Box::new(affectation));
        instructions.push(Box::new(condition));
    }

    let now = Instant::now();
    let _ = interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Without condition loop: {:?}", elapsed.as_secs_f64());
}

#[test]
fn test_speed_condition_loop(){
    test_speed_condition_loop_p1();
    test_speed_condition_loop_p2();
}