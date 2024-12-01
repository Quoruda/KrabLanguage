use std::collections::HashMap;


pub(crate) trait Valuable {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64;
}

pub(crate) struct Number {
    value: f64,
}

impl Number {
    pub(crate) fn new(value: f64) -> Number {
        Number{value}
    }
}

pub(crate)  struct Variable {
    name: String,
}

impl Variable {
    pub(crate) fn new(name: &str) -> Variable {
        Variable{name: name.to_string()}
    }
}


impl Valuable for Number {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        self.value
    }
}

impl Valuable for Variable {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        match variables.get(&self.name) {
            Some(value) => *value,
            None => panic!("Variable not found"),
        }
    }
}



pub(crate) struct Operation {
    left: Box<dyn Valuable>,
    right: Box<dyn Valuable>,
    operator: char,
}

impl Operation {
    pub(crate) fn new(left: Box<dyn Valuable>, right: Box<dyn Valuable>, operator: char) -> Operation {
        Operation{left, right, operator}
    }
}

impl Valuable for Operation {
    fn get_value(&self, variables: &HashMap<String, f64>) -> f64 {
        match self.operator {
            '+' => self.left.get_value(variables) + self.right.get_value(variables),
            '-' => self.left.get_value(variables) - self.right.get_value(variables),
            '*' => self.left.get_value(variables) * self.right.get_value(variables),
            '/' => self.left.get_value(variables) / self.right.get_value(variables),
            _ => panic!("Invalid operator"),
        }
    }
}

pub(crate) struct Affectation {
    variable: String,
    value: Box<dyn Valuable>,
}

impl Affectation {
    pub(crate) fn new(variable: &str, value: Box<dyn Valuable>) -> Affectation {
        Affectation{variable: variable.to_string(), value}
    }
}

pub(crate) trait Instruction {
    fn execute(&self, variables: &mut HashMap<String, f64>);
}

impl Instruction for Affectation {
    fn execute(&self, variables: &mut HashMap<String, f64>) {
        variables.insert(self.variable.clone(), self.value.get_value(variables));
    }
}

impl Instruction for Operation {
    fn execute(&self, variables: &mut HashMap<String, f64>) {
        self.get_value(variables);
    }
}


pub(crate) struct Interpreter {
    running:bool,
    print_errors:bool,
    variables:HashMap<String, f64>,
}

impl Interpreter{
    pub(crate) fn new() -> Interpreter{
        Interpreter{
            running:true,
            print_errors:true,
            variables:HashMap::new(),
        }
    }

    pub(crate) fn get_variable(&self, name: &str) -> Option<&f64> {
        self.variables.get(name)
    }
    pub(crate) fn new_for_tests() -> Interpreter{
        Interpreter{
            running:true,
            print_errors:false,
            variables:HashMap::new(),
        }
    }


    pub(crate) fn execute (&mut self, instruction: &dyn Instruction){
        instruction.execute(&mut self.variables);
    }

}