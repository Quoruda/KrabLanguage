use std::vec::Vec;

pub struct Token{
    token_type: String,
    value: String,
}

impl Token{
    pub fn new(token_type: &str, value: &str) -> Token{
        Token{token_type: token_type.to_string(), value: value.to_string()}
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

pub struct Lexer{

}

impl Lexer{
    pub fn new() -> Lexer{
        Lexer{}
    }

    pub fn lex(&self, input: &str) -> Vec<Token>{
        let tokens = Vec::new();


        return tokens;
    }

}

