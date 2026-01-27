use std::collections::VecDeque;

/// Stack implemented using a single queue with push-time reordering.
///
/// # Intuition
/// After each push, rotate all previous elements behind the new one,
/// making the most recently pushed element always at the front.
///
/// # Approach
/// Use one VecDeque. On push, add to back then rotate `len - 1` elements
/// from front to back, ensuring LIFO order at the front.
///
/// # Complexity
/// - Time: O(n) push, O(1) pop/top/empty
/// - Space: O(n)
struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        let rotations = self.queue.len() - 1;
        for _ in 0..rotations {
            let val = self.queue.pop_front().unwrap();
            self.queue.push_back(val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert!(!stack.empty());
        assert_eq!(stack.pop(), 1);
        assert!(stack.empty());
    }
}
