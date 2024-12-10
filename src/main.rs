mod interpreter;
mod errors;
mod value;
mod parser;
mod lexer;

mod variables;

use std::io;
use std::fs;
use std::env;
use crate::errors::CustomError;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;


fn next_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input
}

fn print_error(error: &CustomError){
    eprintln!("\x1b[31m{}\x1b[0m", error);
}

fn read_file(file_path: &str) -> Result<String, CustomError>{
    let content = fs::read_to_string(file_path);
    match content {
        Ok(content) => Ok(content),
        Err(_) => Err(CustomError::new_file_not_found_error(file_path)),
    }
}

fn terminal_loop(lexer: Lexer, parser: Parser, interpreter: &mut Interpreter){
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
        match parser.parse_instructions(tokens) {
            Ok(inst) => {
                match interpreter.execute_instructions(&inst) {
                    Ok(value) => println!("{:?}", value),
                    Err(error) => print_error(&error),
                };
            }
            Err(error) => {
                print_error(&error);
                continue;
            }
        };
    }
}

fn main() {
    let lexer = Lexer::new();
    let parser = Parser::new();
    let mut interpreter = Interpreter::new();


    let args: Vec<String> = env::args().collect();    
    if args.len() > 1 {
        let file_path = &args[1];
        let content :String;
        match read_file(file_path){
            Ok(c) => content = c,
            Err(error) => {
                print_error(&error);
                return;
            }
        };
        let tokens; 
        
        match lexer.lex(&content){
            Ok(tks) => tokens = tks,
            Err(e) => {print_error(&e); return;}
        }
        let instructions;
        match parser.parse_instructions(tokens){
            Ok(inst) => instructions = inst,
            Err(e) => {print_error(&e); return;}
        }
        match interpreter.execute_instructions(&instructions){
            Ok(_) => (),
            Err(error) => {
                print_error(&error);
            }
        }
        //print the variables
        println!("{:?}", interpreter.variables);
    }else{
        terminal_loop(lexer, parser, &mut interpreter);
    }
}
