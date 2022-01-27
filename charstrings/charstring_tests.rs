mod charstring;

#[test]
fn it_converts_string_to_i64() {
    let ans: i64 = 12345;
    assert_eq!(charstring::atoi("12345"), ans);
}

#[test]
fn it_converts_char_to_lowercase() {
    assert_eq!(charstring::to_lower('A'), 'a');
}
