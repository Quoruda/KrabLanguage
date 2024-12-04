use std::vec::Vec;
use crate::errors::CustomError;

pub struct Token{
    token_type: String,
    value: String,
}

impl Token{

    pub fn new_identifier(value: &str) -> Token{
        Token{
            token_type: "IDENTIFIER".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_assign(value: &str) -> Token{
        Token{
            token_type: "ASSIGN".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_number(value: &str) -> Token{
        Token{
            token_type: "NUMBER".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_string(value: &str) -> Token{
        Token{
            token_type: "STRING".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_operator(value: &str) -> Token{
        Token{
            token_type: "OPERATOR".to_string(),
            value: value.to_string(),
        }
    }

    pub fn get_token_type(&self) -> &str{
        &self.token_type
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn equals(&self, token: &Token) -> bool{
        self.token_type == token.token_type && self.value == token.value
    }

}

impl Clone for Token{
    fn clone(&self) -> Token{
        Token{
            token_type: self.token_type.clone(),
            value: self.value.clone(),
        }
    }
}

pub struct Lexer{
    operators: Vec<char>,
}

impl Lexer{
    pub fn new() -> Lexer{
        Lexer{
            operators: vec!['+', '-', '*', '/'],
        }
    }

    pub fn lex(&self, input: &str) -> Result<Vec<Token>,CustomError>{
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < input.len(){
            let c = input.chars().nth(i).unwrap();
            if c.is_whitespace(){
                i += 1;
                continue;
            }
            if c == '(' || c == ')'  || c == ';'{
                tokens.push(Token::new_operator(&c.to_string()));
                i += 1;
                continue;
            }
            if c.is_alphabetic() || c == '_'{
                let mut j = i;
                while j < input.len() && (input.chars().nth(j).unwrap().is_alphabetic() || input.chars().nth(j).unwrap() == '_' || input.chars().nth(j).unwrap().is_numeric()){
                    j += 1;
                }
                tokens.push(Token::new_identifier(&input[i..j]));
                i = j;
                continue;
            }
            if c.is_numeric() {
                let mut j = i;
                while j < input.len() && (input.chars().nth(j).unwrap().is_numeric() || input.chars().nth(j).unwrap() == '.'){
                    j += 1;
                }
                if input[i..j].matches('.').count() > 1{
                    return Err(CustomError::new_lexer_error("Invalid number"));
                }
                tokens.push(Token::new_number(&input[i..j]));
                i = j;
                continue;
            }
            if c == '"'{
                let mut j = i + 1;
                while j < input.len() && input.chars().nth(j).unwrap() != '"'{
                    j += 1;
                }
                if j == input.len() && input.chars().nth(j-1).unwrap() != '"'{
                    return Err(CustomError::new_lexer_error("String not closed"));
                }
                tokens.push(Token::new_string(&input[i+1..j]));
                i = j + 1;
                continue;
            }
            if self.operators.contains(&c){
                tokens.push(Token::new_operator(&c.to_string()));
                i += 1;
                continue;
            }

            if c == '='{
                tokens.push(Token::new_assign("="));
                i += 1;
                continue;
            }

            return Err(CustomError::new_lexer_error(&format!("Unknown character: {}", c)));
        }
        Ok(tokens)
    }
}

