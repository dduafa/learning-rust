pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
 
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    
    // use <Option> because we can get an item(Some()) or empty(None) when stack is empty.
    pub fn pop(&mut self) -> Option<T> { 
        self.stack.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    // use <Option> because we can get an item(Some()) or empty(None) when stack is empty.
    // Option<T> removes item from the stack which we don't want
    // So we get a reference instead Option<&T>
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    // use usize and isize when it’s related to memory size
    // ie. the size of an object, or indexing a vector,
    pub fn length(&self) -> usize {
        self.stack.len()
    }
}

