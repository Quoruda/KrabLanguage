extern crate KrabLanguage;
use KrabLanguage::interpreter::{StringValue, Interpreter, FloatValue, Variable, Affectation, Operation, IntegerValue, Condition, ConditionBlock, ConditionLoop, Instruction};
use KrabLanguage::value::Value;
use KrabLanguage::errors::CustomError;
use std::time::Instant;

#[test]
fn test_speed(){
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
    interpreter.execute_instructions(&instructions);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed.as_secs_f64());
}