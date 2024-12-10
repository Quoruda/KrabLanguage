use crate::interpreter::{Instruction, Operation, FloatValue, StringValue, Variable, Affectation, Valuable, IntegerValue, Condition, InstructionBlock, ConditionLoop};
use crate::lexer::Token;
use crate::errors::CustomError;

pub struct Parser{

}

impl Parser{
    pub fn new() -> Parser{
        Parser{}
    }

    fn is_value(token: &Token) -> bool{
        let token_type = token.get_token_type();
        token_type == Token::new_identifier("").get_token_type() || token_type == Token::new_number("0.0").get_token_type() || token_type == Token::new_string("").get_token_type()
    }

    fn get_valuable(&self, tokens: Vec<Token>) -> Result<Box<dyn Valuable>, CustomError >{
        if tokens.len() == 0{
            return Err(CustomError::new_parser_error("Value expected but none found"));
        }else if tokens.len() == 1{
            let token_type = tokens[0].get_token_type();
            if token_type == Token::new_identifier("").get_token_type(){
                return Ok(Box::new(Variable::new(tokens[0].get_value())));
            }else if token_type == Token::new_number("0.0").get_token_type(){
                if tokens[0].get_value().contains("."){
                    return Ok(Box::new(FloatValue::new(tokens[0].get_value().parse::<f64>().unwrap())));
                }else{
                    return Ok(Box::new(IntegerValue::new(tokens[0].get_value().parse::<i64>().unwrap())));
                }
            }else if token_type == Token::new_string("").get_token_type(){
                return Ok(Box::new(StringValue::new(tokens[0].get_value())));
            }
            return Err(CustomError::new_parser_error(&format!("Unexpected value: {}", tokens[0].get_value())));
        }else{
            if tokens.len() == 3{
                if Self::is_value(&tokens[0]) && Self::is_value(&tokens[2]){
                    if tokens[1].get_token_type() == Token::new_operator("+").get_token_type(){
                        let left = self.get_valuable(vec![tokens[0].clone()])?;
                        let right = self.get_valuable(vec![tokens[2].clone()])?;
                        let operator:char = tokens[1].get_value().chars().nth(0).unwrap();
                        return Ok(Box::new(Operation::new(left, right, operator)));
                    }else if tokens[1].get_token_type() == Token::new_comparator(">").get_token_type(){
                        let left = self.get_valuable(vec![tokens[0].clone()])?;
                        let right = self.get_valuable(vec![tokens[2].clone()])?;
                        let comparator:char;
                        match tokens[1].get_value() {
                            ">" => comparator = '>',
                            "<" => comparator = '<',
                            _ => return Err(CustomError::new_parser_error("Not implemented yet")),
                        }
                        return Ok(Box::new(Condition::new(left, right, comparator)));
                    }

                }

            }
            return Err(CustomError::new_parser_error("Not implemented yet"));
        }
    }

    pub fn parse_instructions(&self, tokens: Vec<Token>) -> Result<Vec<Box<dyn Instruction>>, CustomError>{
        let mut instructions = Vec::new();
        let mut i = 0;
        let mut bracket_count = 0;
        while i < tokens.len(){
            let mut j = i;
            while j < tokens.len() && (!tokens[j].equals(&Token::new_semicolon()) || bracket_count != 0){
                if tokens[j].equals(&Token::new_bracket("{")){
                    bracket_count += 1;
                }else if tokens[j].equals(&Token::new_bracket("}")){
                    bracket_count -= 1;
                }
                j += 1;
            }
            let instruction = self.parse(tokens[i..j].to_vec());
            match instruction{
                Ok(instruction) => instructions.push(instruction),
                Err(error) => return Err(error),
            }
            i = j + 1;
        }
        return Ok(instructions);
    }


    pub fn parse(&self, tokens: Vec<Token>) -> Result<Box<dyn Instruction>, CustomError>{
        if tokens.len() == 0{
            return Err(CustomError::new_parser_error("No tokens to parse"));
        }
        if tokens[0].get_token_type() == Token::new_identifier("").get_token_type(){
            let variable = tokens[0].get_value();
            if tokens.len() > 1 && tokens[1].equals(&Token::new_assign("=")){
                if tokens.len() < 3{
                    return Err(CustomError::new_parser_error("Value expected but none found"));
                }
                let result = self.get_valuable(tokens[2..].to_vec());
                match result{
                    Ok(value) => return Ok(Box::new( Affectation::new(&variable.to_string(), value))),
                    Err(error) => return Err(error),
                }
            }else {
                return Ok(Box::new(Variable::new(variable)));
            }
        }
        
        if tokens[0].equals(&Token::new_keyword("while")){
            let tks = tokens[1..].to_vec();
            let mut i = 0;
            while i < tks.len() && !tks[i].equals(&Token::new_bracket("{")){
                i += 1;
            }
            if i == tks.len(){
                return Err(CustomError::new_parser_error("'{' expected but none found"));
            }
            let condition;
            match self.get_valuable(tks[0..i].to_vec()){
                Ok(c) => condition = c,
                Err(error) => return Err(error),
            }
            let mut j = i + 1;
            while j < tks.len() && !tks[j].equals(&Token::new_bracket("}")){
                j += 1;
            }
             if j == tks.len(){
                return Err(CustomError::new_parser_error("'}' expected but none found"));
            }
            let block = self.parse_instructions(tks[i+1..j].to_vec());
            match block{
                Ok(block) => return Ok(Box::new(ConditionLoop::new(condition, InstructionBlock::new(block)))),
                Err(error) => return Err(error),
            } 
        }
        
        match self.get_valuable(tokens.clone()){
            Ok(v) => {
                let instruction: Box<dyn Instruction> = Box::new(v);
                return Ok(instruction);
            }
            Err(_) => {}
        }
        return Err(CustomError::new_parser_error(&format!("Unexpected token: {}", tokens[0].get_value())));
    }

}