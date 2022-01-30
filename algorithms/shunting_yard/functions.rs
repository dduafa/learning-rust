pub fn is_digit(c: char) -> bool {
    let c = c as u8;
    c >= b'0' && c <= b'9'
}

pub fn is_operator(c: char) -> bool {
    let operators = vec!['^', '/', '*', '+', '-'];
    for op in operators.iter() {
        if *op == c {
            return true;
        }
    }
    return false;
}