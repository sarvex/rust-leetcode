impl Solution {
    /// Validates if pushed/popped sequences correspond to valid stack operations.
    ///
    /// # Intuition
    /// Simulate the stack: push elements and pop whenever the stack top
    /// matches the next expected popped element.
    ///
    /// # Approach
    /// Iterate through pushed elements, pushing each onto the stack. After
    /// each push, pop as long as the top matches the current popped element.
    /// Valid if all popped elements are consumed.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the simulated stack
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut j = 0;
        for &x in &pushed {
            stack.push(x);
            while stack.last() == Some(&popped[j]) {
                stack.pop();
                j += 1;
            }
        }
        j == popped.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1],
        ));
    }

    #[test]
    fn test_invalid() {
        assert!(!Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2],
        ));
    }

    #[test]
    fn test_single() {
        assert!(Solution::validate_stack_sequences(vec![1], vec![1]));
    }
}
