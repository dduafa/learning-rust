pub fn atoi(s: &str) -> i64 {
    let mut n = 0;

    for c in s.chars() {
        let tc = c as u8;

        if tc >= b'0' && tc <= b'9' {
            n = (10 * n) + ((tc - b'0') as i64);
        }
    }
    return n;
}


pub fn to_lower(c: char) -> char {
    let c = c as u8;

    if c >= b'A' && c <= b'Z' {
        return (c + 32) as char;
    }
    return c as char;
}


pub fn to_upper(c: char) -> char {
    let c = c as u8;

    if c >= b'a' && c <= b'z' {
        return (c - 32) as char;
    }
    return c as char;
}


pub fn print_chars_from_bytes(s: &String) {
    for &item in s.as_bytes().iter()  {
        println!("{}", item as char)
    }
}   


pub fn print_chars_from_chars(s: &String) {
    for c in s.chars() {
        println!("{}", c)
    }
}  