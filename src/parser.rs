use crate::interpreter::{Instruction, Operation, FloatValue, StringValue, Variable};
use crate::lexer::Token;

pub struct Parser{

}

impl Parser{
    pub fn new() -> Parser{
        Parser{}
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Box<dyn Instruction>{
        for token in tokens{
            println!("Token: {}", token.get_value());
        }
        return Box::new(Operation::new(Box::new(FloatValue::new(20.0)), Box::new(FloatValue::new(20.0)), '+'));
    }

}