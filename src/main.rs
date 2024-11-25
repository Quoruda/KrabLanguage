use std::io;

static mut RUNNING:bool = true;

fn main() {
    while unsafe {RUNNING} {
        next_line();
    }
}

fn next_line(){
    print!(">> ");
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();
    tokenised_input(trimmed_input);
}

fn tokenised_input(input: &str){
    let mut tokens: Vec<String> = Vec::new();
    let mut i_start = 0;
    let mut i_end = 0;
    let variable_regex = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();


    for c in input.chars() {
        if c == ' ' {
            tokens.push(input.chars().skip(i_start).take(i_end-i_start).collect());
            i_start = i_end + 1;
        }
        i_end += 1;
    }
    if i_start != i_end{
        tokens.push(input.chars().skip(i_start).take(i_end-i_start).collect());
    }
    //temp exit
    if tokens[0] == "exit" {
        unsafe {
            RUNNING = false;
        };
    }

    for token in tokens {
        //check if token is a variable
        if variable_regex.is_match(&token){
            println!("Variable: {}", token);
        } else {
            eprintln!("\x1b[31mInvalid token: {}\x1b[0m", token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){
    }
}

