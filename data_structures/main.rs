mod stack;
mod queue;

fn main() {
    let mut stack: stack::Stack<isize> = stack::Stack::new();
    stack.push(1);
    stack.push(10);
    stack.push(46);
    println!("{:?}", stack.peek().unwrap());
    println!("{:?}", stack.pop());

    let mut queue: queue::Queue<&str> = queue::Queue::new();
    queue.enqueue("First In");
    queue.enqueue("Second In");
    println!("{:?}", queue.peek());
}
