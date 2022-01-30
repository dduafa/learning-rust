use std::collections::HashMap;

let mut operators: HashMap<char, Operator> = HashMap::new();   

#[derive(Debug)]
struct Operator {
    operator: char,
    precedence: u8,
    associativity: &'static str,
}

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
