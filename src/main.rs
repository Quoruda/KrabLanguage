use std::ops::Deref;

mod interpreter;
mod errors;
mod value;
mod parser;
mod lexer;

fn next_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input
}

fn main() {
    let lexer = lexer::Lexer::new();
    let parser = parser::Parser::new();
    let mut interpreter = interpreter::Interpreter::new();
    loop {
        print!(">> ");
        let input = next_line();
        let tokens;
        match lexer.lex(&input) {
            Ok(tks) => tokens= tks,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };
        let instruction;
        match parser.parse(tokens) {
            Ok(inst) => instruction = inst,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };
        match interpreter.execute(instruction.deref()) {
            Ok(value) => println!("{:?}", value),
            Err(error) => println!("{}", error),
        };

    }
}
