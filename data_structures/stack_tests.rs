mod stack;

#[test]
fn it_returns_item_on_top_of_stack() {
    let mut stack: stack::Stack<isize> = stack::Stack::new();
    stack.push(1);
    stack.push(10);
    stack.push(46);
    assert_eq!(stack.peek().unwrap(), 46);
}
