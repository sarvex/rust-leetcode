use std::collections::VecDeque;

/// Zigzag iterator alternating between two vectors using deques.
///
/// # Intuition
/// Use VecDeque for efficient front removal. Alternate between the two deques,
/// falling through to the non-empty one when one is exhausted.
///
/// # Approach
/// 1. Store both vectors as VecDeques.
/// 2. Track a flag to alternate between them.
/// 3. On next(), pop from the current deque; if empty, pop from the other.
///
/// # Complexity
/// - Time: O(1) per next/has_next
/// - Space: O(n + m)
struct ZigzagIterator {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    use_first: bool,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        Self {
            q1: VecDeque::from(v1),
            q2: VecDeque::from(v2),
            use_first: true,
        }
    }

    fn next(&mut self) -> i32 {
        let val = if self.use_first {
            self.q1.pop_front().or_else(|| self.q2.pop_front())
        } else {
            self.q2.pop_front().or_else(|| self.q1.pop_front())
        };
        self.use_first = !self.use_first;
        val.unwrap()
    }

    fn has_next(&self) -> bool {
        !self.q1.is_empty() || !self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternating_output() {
        let mut iter = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1, 3, 2, 4, 5, 6]);
    }

    #[test]
    fn one_empty() {
        let mut iter = ZigzagIterator::new(vec![], vec![1, 2, 3]);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next());
        }
        assert_eq!(result, vec![1, 2, 3]);
    }
}
