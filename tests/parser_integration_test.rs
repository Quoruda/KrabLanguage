extern crate krab_language;
use krab_language::errors::CustomError;
use krab_language::lexer::{Lexer, Token};
use krab_language::parser::Parser;
use krab_language::interpreter::Interpreter;
use krab_language::value::Value;

fn eq_values(value1: &Value, value2: &Value) -> bool {
    match value1.eq(value2) {
        Ok(b) => b,
        Err(_) => false
    }
}

#[test]
fn test_affectation(){
    let parser = Parser::new();
    let mut interpreter = Interpreter::new();
    let tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20"),
        Token::new_semicolon()
    ];
    match parser.parse_instructions(tokens){
        Ok(instructions) => {
            match interpreter.execute_instructions(&instructions){
                Ok(_) => {
                    match interpreter._get_variable("a"){
                        Ok(value) => assert!(eq_values(&value, &Value::Integer(20))),
                        Err(_) => assert!(false)
                    }
                }
                Err(_) => assert!(false)
            }
        },
        Err(_) => assert!(false)
    }
}

#[test]
fn test_condition_loop(){
    let parser = Parser::new();
    let mut interpreter = Interpreter::new();
    let tokens = vec![
        Token::new_identifier("i"),
        Token::new_assign("="),
        Token::new_number("0"),
        Token::new_semicolon(),
        Token::new_keyword("while"),
        Token::new_identifier("i"),
        Token::new_comparator("<"),
        Token::new_number("10"),
        Token::new_bracket("{"),
        Token::new_identifier("i"),
        Token::new_assign("="),
        Token::new_identifier("i"),
        Token::new_operator("+"),
        Token::new_number("1"),
        Token::new_semicolon(),
        Token::new_bracket("}"),
        Token::new_semicolon()
    ];
    match parser.parse_instructions(tokens){
        Ok(instructions) => {
            match interpreter.execute_instructions(&instructions){
                Ok(_) => {
                    match interpreter._get_variable("i"){
                        Ok(value) => assert!(eq_values(&value, &Value::Integer(10))),
                        Err(_) => assert!(false)
                    }
                }
                Err(_) => assert!(false)
            }
        },
        Err(_) => assert!(false)
    }
}



