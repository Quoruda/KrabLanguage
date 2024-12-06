extern crate KrabLanguage;
use KrabLanguage::errors::CustomError;
use KrabLanguage::lexer::{Lexer, Token};
use KrabLanguage::parser::Parser;

#[test]
fn affectation_test() {
    let tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20")
    ];
    let parser = Parser::new();
    let instruction = parser.parse(tokens);
}
