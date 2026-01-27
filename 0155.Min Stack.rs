/// A stack that supports push, pop, top, and retrieving the minimum in O(1).
///
/// # Intuition
/// Maintain a secondary stack that tracks the current minimum. When pushing,
/// also push to the min stack if the value is <= the current minimum.
///
/// # Approach
/// Use two `Vec<i32>` stacks: one for values and one for minimums.
/// The min stack only grows when a new minimum (or equal) is pushed.
///
/// # Complexity
/// - Time: O(1) per operation
/// - Space: O(n) for both stacks
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if *self.min_stack.last().unwrap() == val {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn duplicate_minimums() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(1);
        assert_eq!(stack.get_min(), 1);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
    }
}
