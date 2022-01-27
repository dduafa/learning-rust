mod charstring;

fn main() {
    let s: &str = "12345";
    println!("converting string `{}` to integer: {}", s, charstring::atoi(s));

    let v: char = 'A';
    println!("converting upper char `{}` to lower char: {}", v, charstring::to_lower(v));
}