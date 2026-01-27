impl Solution {
    /// Computes days until a warmer temperature using a monotonic stack.
    ///
    /// # Intuition
    /// A decreasing stack of indices tracks temperatures waiting for a warmer
    /// day. When a warmer temperature appears, it resolves all smaller entries.
    ///
    /// # Approach
    /// Traverse right-to-left, maintaining a stack of indices with decreasing
    /// temperatures. For each day, pop entries that are not warmer, then the
    /// stack top gives the answer.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the stack
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in (0..n).rev() {
            while stack
                .last()
                .is_some_and(|&top| temperatures[top] <= temperatures[i])
            {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                result[i] = (top - i) as i32;
            }
            stack.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_descending() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn test_ascending() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
