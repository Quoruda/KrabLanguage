use std::vec::Vec;
use crate::errors::CustomError;
use std::fmt::Debug;

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

    pub fn new_comparator(value: &str) -> Token{
        Token{
            token_type: "COMPARATOR".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_semicolon() -> Token{
        Token{
            token_type: "SEMICOLON".to_string(),
            value: ";".to_string(),
        }
    }

    pub fn new_parenthesis(value: &str) -> Token{
        Token{
            token_type: "PARENTHESIS".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_bracket(value: &str) -> Token{
        Token{
            token_type: "BRACKET".to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_keyword(value: &str) -> Token{
        Token{
            token_type: "KEYWORD".to_string(),
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

impl Debug for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" ,self.value)
    }
}

pub struct Lexer{
    operators: Vec<char>,
    comparator: Vec<char>,
    keywords: Vec<String>,
}

impl Lexer{
    pub fn new() -> Lexer{
        Lexer{
            operators: vec!['+', '-', '*', '/'],
            comparator: vec!['>', '<'],
            keywords: vec!["while".to_string(), "if".to_string(), "else".to_string()]
        }
    }

    pub fn lex(&self, input: &String) -> Result<Vec<Token>,CustomError>{
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = input.chars().collect();
        while i < chars.len(){
            let c = chars[i];
            if c == '#'{
                let mut j = i+1;
                while j < chars.len() && chars[j] != '#'{
                    j += 1;
                }
                if j >= chars.len() {
                    return Err(CustomError::new_lexer_error("Comment not closed"));
                }
                i = j+1;
                continue;
            }
            if c.is_whitespace() || c == '\n' || c == '\t' || c == '\r'{
                i += 1;
                continue;
            }
            if c == '(' || c == ')'{
                tokens.push(Token::new_parenthesis(&c.to_string()));
                i += 1;
                continue;
            }
            if c == '{' || c == '}'{
                tokens.push(Token::new_bracket(&c.to_string()));
                i += 1;
                continue;
            }
            if c == ';'{
                tokens.push(Token::new_semicolon());
                i += 1;
                continue;
            }
            if c.is_ascii_alphabetic() || c == '_'{
                let mut j = i;
                while j < chars.len() && (chars[j].is_ascii_alphabetic() || chars[j] == '_' || chars[j].is_numeric()){
                    j += 1;
                }
                if self.keywords.contains(&chars[i..j].iter().collect::<String>()){
                    tokens.push(Token::new_keyword(&chars[i..j].iter().collect::<String>()));
                }else{
                    tokens.push(Token::new_identifier(&chars[i..j].iter().collect::<String>()));
                }
                i = j;
                continue;
            }
            if c.is_numeric() {
                let mut j = i;
                while j < chars.len() && (chars[j].is_numeric() || chars[j] == '.'){
                    j += 1;
                }
                if chars[i..j].iter().collect::<String>().matches('.').count() > 1{
                    return Err(CustomError::new_lexer_error("Invalid number"));
                }
                tokens.push(Token::new_number(&chars[i..j].iter().collect::<String>()));
                i = j;
                continue;
            }
            if c == '"'{
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"'{
                    j += 1;
                }
                if j == chars.len() && chars[j-1] != '"'{
                    return Err(CustomError::new_lexer_error("String not closed"));
                }
                tokens.push(Token::new_string(&chars[i+1..j].iter().collect::<String>()));
                i = j + 1;
                continue;
            }
            if self.operators.contains(&c){
                tokens.push(Token::new_operator(&c.to_string()));
                i += 1;
                continue;
            }
            if self.comparator.contains(&c){
                let mut j = i;
                while j < chars.len() && (self.comparator.contains(&chars[j]) || chars[j] == '='){
                    j += 1;
                }
                tokens.push(Token::new_comparator(&chars[i..j].iter().collect::<String>()));
                i = j;
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

