extern crate krab_language;

use krab_language::errors::CustomError;
use krab_language::lexer::{Lexer, Token};

fn compare_tokens(tokens: Vec<Token>, expected_tokens: Vec<Token>) -> bool {
    if tokens.len() != expected_tokens.len() {
        return false;
    }
    for i in 0..tokens.len() {
        if !tokens[i].equals(&expected_tokens[i]) {
            return false;
        }
    }
    return true;
}

#[test]
fn affectation_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"a = 20".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn string_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"\"hello world\" \"ok\"".to_string());
    let expected_tokens = vec![
        Token::new_string("hello world"),
        Token::new_string("ok")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn number_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"123 456 78.9".to_string());
    let expected_tokens = vec![
        Token::new_number("123"),
        Token::new_number("456"),
        Token::new_number("78.9")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn operator_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"+ - * /    ++".to_string());
    let expected_tokens = vec![
        Token::new_operator("+"),
        Token::new_operator("-"),
        Token::new_operator("*"),
        Token::new_operator("/"),
        Token::new_operator("+"),
        Token::new_operator("+")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn identifier_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"hello_2 world158 ___r___".to_string());
    let expected_tokens = vec![
        Token::new_identifier("hello_2"),
        Token::new_identifier("world158"),
        Token::new_identifier("___r___")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn parenthesis_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"(a + b) * (c - d)".to_string());
    let expected_tokens = vec![
        Token::new_parenthesis("("),
        Token::new_identifier("a"),
        Token::new_operator("+"),
        Token::new_identifier("b"),
        Token::new_parenthesis(")"),
        Token::new_operator("*"),
        Token::new_parenthesis("("),
        Token::new_identifier("c"),
        Token::new_operator("-"),
        Token::new_identifier("d"),
        Token::new_parenthesis(")")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn semicolon_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"a = 20; b = 30;".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20"),
        Token::new_semicolon(),
        Token::new_identifier("b"),
        Token::new_assign("="),
        Token::new_number("30"),
        Token::new_semicolon()
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn invalid_character(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"ù".to_string());
    match tokens {
        Ok(_) => assert!(false),
        Err(err) => assert!(err._equals(&CustomError::new_lexer_error("Unknown character: ù")))
    }
    let tokens = lexer.lex(&"\"ù\"".to_string());
    let expected_tokens = vec![
        Token::new_string("ù")
    ];
    match tokens {
        Ok(_) => assert!(compare_tokens(tokens.unwrap(), expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn invalid_string(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"\"hello".to_string());
    match tokens {
        Ok(_) => assert!(false),
        Err(err) => assert!(err._equals(&CustomError::new_lexer_error("String not closed")))
    }
}

#[test]
fn invalid_number(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"123.456.789".to_string());
    match tokens {
        Ok(_) => assert!(false),
        Err(err) => assert!(err._equals(&CustomError::new_lexer_error("Invalid number")))
    }
}

#[test]
fn test_comparator(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"a > b".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_comparator(">"),
        Token::new_identifier("b")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
    let tokens = lexer.lex(&"a >= b".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_comparator(">="),
        Token::new_identifier("b")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
    let tokens = lexer.lex(&"a < b".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_comparator("<"),
        Token::new_identifier("b")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
    let tokens = lexer.lex(&"a <= b".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_comparator("<="),
        Token::new_identifier("b")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_bracket(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"{a = 20}".to_string());
    let expected_tokens = vec![
        Token::new_bracket("{"),
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20"),
        Token::new_bracket("}")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_while(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"while a < b { a = a + 1; }".to_string());
    let expected_tokens = vec![
        Token::new_keyword("while"),
        Token::new_identifier("a"),
        Token::new_comparator("<"),
        Token::new_identifier("b"),
        Token::new_bracket("{"),
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_identifier("a"),
        Token::new_operator("+"),
        Token::new_number("1"),
        Token::new_semicolon(),
        Token::new_bracket("}")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_line_feed(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"a = 20\nb = 30".to_string());
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20"),
        Token::new_identifier("b"),
        Token::new_assign("="),
        Token::new_number("30")
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}

#[test]
fn test_else(){
    let lexer = Lexer::new();
    let tokens = lexer.lex(&"if a > b { a = a + 1; } else { a = a - 1; };".to_string());
    let expected_tokens = vec![
        Token::new_keyword("if"),
        Token::new_identifier("a"),
        Token::new_comparator(">"),
        Token::new_identifier("b"),
        Token::new_bracket("{"),
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_identifier("a"),
        Token::new_operator("+"),
        Token::new_number("1"),
        Token::new_semicolon(),
        Token::new_bracket("}"),
        Token::new_keyword("else"),
        Token::new_bracket("{"),
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_identifier("a"),
        Token::new_operator("-"),
        Token::new_number("1"),
        Token::new_semicolon(),
        Token::new_bracket("}"),
        Token::new_semicolon()
    ];
    match tokens {
        Ok(tokens) => assert!(compare_tokens(tokens, expected_tokens)),
        Err(_) => assert!(false)
    }
}