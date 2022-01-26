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

#[cfg(test)]
mod tests {

    #[test]
    fn it_converts_string_to_i64() {
        let ans: i64 = 12345;
        assert_eq!(atoi("12345"), ans);
    }
}