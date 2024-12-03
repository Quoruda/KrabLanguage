use crate::interpreter::Instruction;
use crate::interpreter::Operation;
use crate::interpreter::FloatValue;
use crate::lexer::Token;

struct Parser{

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