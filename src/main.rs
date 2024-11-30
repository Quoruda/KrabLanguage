use std::io;
use std::collections::HashMap;

static mut RUNNING:bool = true;

static mut PRINT_ERRORS:bool = true;
static KEYWORDS: [&str; 2] = ["if", "while"];
static OPERATORS: [&str; 6] = ["+", "-", "*", "/", "%", "="];

struct Token {
    token_type: String,
    token_value: String,
}


fn main() {
    let mut variables: HashMap<String, i64> = HashMap::new();
    loop {
        let tokens: Vec<Token> = next_line();
        runner(tokens, &mut variables);
        println!("{:?}", variables);
    }
}

fn print_error(error: &str){
    unsafe {
        if PRINT_ERRORS {
            eprintln!("\x1b[31m{}\x1b[0m", error);
        }
    }
}

fn next_line() -> Vec<Token>{
    print!(">>>");
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();
    let tokens: Vec<Token> = lexer(trimmed_input);
    return tokens;
}



fn lexer(input: &str)->Vec<Token>{
    let mut words: Vec<String> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut i_start = 0;
    let mut i_end = 0;
    let variable_regex = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

    for c in input.chars() {
        if c == ' ' {
            if i_start != i_end{
                words.push(input.chars().skip(i_start).take(i_end-i_start).collect());
            }
            i_start = i_end + 1;
        }else if c == '(' || c == ')'{
            if i_start != i_end{
                words.push(input.chars().skip(i_start).take(i_end-i_start).collect());
            }
            words.push(c.to_string());
            i_start = i_end + 1;
        }else if OPERATORS.contains(&c.to_string().as_str()){
            if i_start != i_end{
                words.push(input.chars().skip(i_start).take(i_end-i_start).collect());
            }
            words.push(c.to_string());
            i_start = i_end + 1;
        }
        i_end += 1;
    }
    if i_start != i_end{
        words.push(input.chars().skip(i_start).take(i_end-i_start).collect());
    }
    //temp exit
    if words[0] == "exit" {
        unsafe {
            RUNNING = false;
        };
    }

    let mut nb_errors = 0;

    for token in words {
        //check if token is a keyword
        if KEYWORDS.contains(&token.as_str()){
            println!("Keyword: {}", token);

        //check if token is an identifier
        }else if variable_regex.is_match(&token){
            tokens.push(Token{token_type: "Identifier".to_string(), token_value: token});
        //check if token is a number
        } else if token.parse::<f64>().is_ok(){
            tokens.push(Token{token_type: "Number".to_string(), token_value: token});
        //check if token is an assignment
        } else if token == "=" {
            tokens.push(Token{token_type: "Assigment".to_string(), token_value: token});
        //check if token is an operator
        } else if OPERATORS.contains(&token.as_str()){
            tokens.push(Token{token_type: "Operator".to_string(), token_value: token});
        //invalid token
        } else {
            nb_errors += 1;
            print_error(&format!("Invalid token: {}", token));
        }
    }


    if nb_errors == 0 {
       return tokens;
    }
    else {
        return Vec::new();
    }

}

/*
fn parser(tokens: Vec<Token>){

}

 */

fn variable_exists(variable: &str, variables: &HashMap<String, i64>)->bool{
    variables.contains_key(variable)
}

fn get_value(token: &Token, variables: &HashMap<String, i64>) ->Result<i64, String>{
    if token.token_type == "Identifier"{
        if variable_exists(&token.token_value, variables){
            Ok(*variables.get(&token.token_value).unwrap())
        } else {
            Err(format!("Variable '{}' not found", token.token_value))
        }
    }else if token.token_type == "Number"{
        Ok(token.token_value.parse::<i64>().unwrap())
    }else{
        Err(format!("Invalid token: {}", token.token_value))
    }
}

fn runner(tokens: Vec<Token>, variables:  &mut HashMap<String, i64>){
    if tokens[0].token_type == "Identifier" && tokens[1].token_type == "Assigment"{
        let mut i = 2;
        let mut expected_value:bool = true;
        let mut value:i64 = 0;
        let mut operator:String;

        while i < tokens.len(){
            if expected_value{
                match get_value(&tokens[i], variables) {
                    Ok(v) => value = v,
                    Err(e) => {
                        print_error(&e);
                        return;
                    }
                }
                expected_value = false;
            }else{
                operator = tokens[i].token_value.clone();
                i += 1;
                if i == tokens.len(){
                    print_error("Expected value after operator");
                    return;
                }
                match get_value(&tokens[i], variables) {
                    Ok(v) => {
                        if operator == "+"{
                            value += v;
                        }else if operator == "-"{
                            value -= v;
                        }else if operator == "*"{
                            value *= v;
                        }else{
                            print_error(format!("Invalid operator: '{}'", operator).as_str());
                            return;
                        }
                    },
                    Err(e) => {
                        print_error(&e);
                        return;
                    }
                }
            }
            i += 1;
        }
        variables.insert(tokens[0].token_value.clone(), value);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn setup(){
        unsafe {
            RUNNING = true;
            PRINT_ERRORS = false;
        }
    }

    #[test]
    fn affectation_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 5");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &5);
    }

    #[test]
    fn multiple_affectation_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 5");
        runner(tokens, &mut variables);
        let tokens = lexer("a = 4");
        runner(tokens, &mut variables);
        let tokens = lexer("a = 8");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &8);
    }

    #[test]
    fn affection_with_variable_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 5");
        runner(tokens, &mut variables);
        let tokens = lexer("b = a");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &5);
    }

    #[test]
    fn affectation_with_non_existent_variable(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("b = a");
        runner(tokens, &mut variables);
        assert_eq!(variable_exists("b", &variables), false);
    }

    #[test]
    fn non_exchange_value(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 5");
        runner(tokens, &mut variables);
        let tokens = lexer("b = a");
        runner(tokens, &mut variables);
        let tokens = lexer("a = 8");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &5);
        assert_eq!(variables.get("a").unwrap(), &8);
    }

    #[test]
    fn operations_without_variable_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 4 + 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &6);
        let tokens = lexer("a = 4 - 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &2);
        let tokens = lexer("a = 4 * 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &8);
    }

    #[test]
    fn operations_with_variable_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 4");
        runner(tokens, &mut variables);
        let tokens = lexer("b = a + 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &6);
        let tokens = lexer("b = a - 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &2);
        let tokens = lexer("b = a * 2");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &8);
        assert_eq!(variables.get("a").unwrap(), &4);
    }

    #[test]
    fn invalid_token_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 4 + 2 +");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a"), None);
    }

    #[test]
    fn multiple_operations_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 4 + 2 - 3 + 4");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("a").unwrap(), &7);
    }

    #[test]
    fn multiple_operations_with_variable_test(){
        setup();
        let mut variables: HashMap<String, i64> = HashMap::new();
        let tokens = lexer("a = 4");
        runner(tokens, &mut variables);
        let tokens = lexer("b = a + a - a + a");
        runner(tokens, &mut variables);
        assert_eq!(variables.get("b").unwrap(), &8);
    }
}

