pub struct Stack<T> {
    s: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.s.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.s.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.s.last()
    }
}
