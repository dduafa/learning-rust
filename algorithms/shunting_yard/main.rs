use std::collections::HashMap;
mod stack;
mod functions;
mod constants;

fn main() {
    let expression: Vec<char> = "3+4*2/(1-5)^2^3".chars().collect();
    assert_eq!(
        shunting_yard(expression),
        ['3', '4', '2', '*', '1', '5', '-', '2', '3', '^', '^', '/', '+']
    );
}

fn shunting_yard(expression: Vec<char>) -> Vec<char> {
    #[derive(Debug)]
    struct Operator {
        operator: char,
        precedence: u8,
        associativity: &'static str,
    }

    let mut operators: HashMap<char, Operator> = HashMap::new();   

    operators.insert('^', Operator {
        operator: '^',
        precedence: 4,
        associativity: "right",
    });
    operators.insert('*', Operator {
        operator: '*',
        precedence: 3,
        associativity: "left",
    });
    operators.insert('/', Operator {
        operator: '/',
        precedence: 3,
        associativity: "left",
    });
    operators.insert('+', Operator {
        operator: '+',
        precedence: 2,
        associativity: "left",
    });
    operators.insert('-', Operator {
        operator: '-',
        precedence: 2,
        associativity: "left",
    });

    let mut stack: stack::Stack<char> = stack::Stack::new();
    let mut output: Vec<char> = Vec::new();
    
    for ch in expression.iter() {
        if ch.is_numeric() {
            output.push(*ch);
        } else if functions::is_operator(*ch) {
            while 
                !stack.is_empty() &&
                functions::is_operator(*stack.peek().unwrap()) &&
                operators.get(ch).unwrap().precedence <= 
                operators.get(stack.peek().unwrap()).unwrap().precedence &&
                operators.get( stack.peek().unwrap() ).unwrap().associativity == "left"
            {            
                output.push(stack.pop().unwrap());
            }
            stack.push(*ch);
        } else if *ch == constants::LEFT_PAREN {
            stack.push(*ch);
        } else if *ch == constants::RIGHT_PAREN {
            while !stack.is_empty() && *stack.peek().unwrap() != constants::LEFT_PAREN {
                output.push(stack.pop().unwrap());
            }
            stack.pop();
        }
    }

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }

    return output;
}