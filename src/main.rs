use std::io;
use std::ops::Deref;
use errors::CustomError;

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

fn print_error(error: &CustomError){
    eprintln!("\x1b[31m{}\x1b[0m", error);
}

fn main() {
    let lexer = lexer::Lexer::new();
    let parser = parser::Parser::new();
    let mut interpreter = interpreter::Interpreter::new();
    loop {
        print!(">>>");
        io::Write::flush(&mut io::stdout()).expect("Flush failed!");
        let input = next_line();
        let tokens;
        match lexer.lex(&input) {
            Ok(tks) => tokens= tks,
            Err(error) => {
                print_error(&error);
                continue;
            }
        };
        let instruction;
        match parser.parse(tokens) {
            Ok(inst) => instruction = inst,
            Err(error) => {
                print_error(&error);
                continue;
            }
        };
        match interpreter.execute(instruction.deref()) {
            Ok(value) => println!("{:?}", value),
            Err(error) => print_error(&error),
        };

    }
}
