struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    /// Queue implemented using two stacks with lazy transfer.
    ///
    /// # Intuition
    /// Elements pushed to one stack are in LIFO order. Transferring to a second
    /// stack reverses them to FIFO order. Transfer lazily only when needed.
    ///
    /// # Approach
    /// Push to `input` stack. On pop/peek, if `output` is empty, transfer all
    /// elements from `input` to `output`. This amortizes the cost.
    ///
    /// # Complexity
    /// - Time: O(1) amortized per operation
    /// - Space: O(n)
    fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.transfer();
        self.output.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.transfer();
        *self.output.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }

    fn transfer(&mut self) {
        if self.output.is_empty() {
            while let Some(val) = self.input.pop() {
                self.output.push(val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert!(!queue.empty());
        assert_eq!(queue.pop(), 2);
        assert!(queue.empty());
    }
}
