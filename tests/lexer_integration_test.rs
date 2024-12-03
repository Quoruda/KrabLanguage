extern crate KrabLanguage;
use KrabLanguage::lexer::{Lexer, Token};

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
    let tokens = lexer.lex("a = 20");
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn string_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("\"hello world\"");
    let expected_tokens = vec![
        Token::new_string("hello world")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn number_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("123 456 78.9");
    let expected_tokens = vec![
        Token::new_number("123"),
        Token::new_number("456"),
        Token::new_number("78.9")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn operator_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("+ - * /    ++");
    let expected_tokens = vec![
        Token::new_operator("+"),
        Token::new_operator("-"),
        Token::new_operator("*"),
        Token::new_operator("/"),
        Token::new_operator("+"),
        Token::new_operator("+")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn identifier_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("hello_2 world158 ___r___");
    let expected_tokens = vec![
        Token::new_identifier("hello_2"),
        Token::new_identifier("world158"),
        Token::new_identifier("___r___")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn parenthesis_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("(a + b) * (c - d)");
    let expected_tokens = vec![
        Token::new_operator("("),
        Token::new_identifier("a"),
        Token::new_operator("+"),
        Token::new_identifier("b"),
        Token::new_operator(")"),
        Token::new_operator("*"),
        Token::new_operator("("),
        Token::new_identifier("c"),
        Token::new_operator("-"),
        Token::new_identifier("d"),
        Token::new_operator(")")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}

#[test]
fn semicolon_test() {
    let lexer = Lexer::new();
    let tokens = lexer.lex("a = 20; b = 30;");
    let expected_tokens = vec![
        Token::new_identifier("a"),
        Token::new_assign("="),
        Token::new_number("20"),
        Token::new_operator(";"),
        Token::new_identifier("b"),
        Token::new_assign("="),
        Token::new_number("30"),
        Token::new_operator(";")
    ];
    assert!(compare_tokens(tokens, expected_tokens));
}