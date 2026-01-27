impl Solution {
    /// Counts visible people to the right using a monotonic stack.
    ///
    /// # Intuition
    /// A person can see another if no taller person stands between them.
    /// Processing right-to-left with a decreasing stack tracks visibility.
    ///
    /// # Approach
    /// 1. Traverse from right to left.
    /// 2. Pop shorter people from the stack (each contributes +1 visibility).
    /// 3. If the stack is non-empty, the person also sees the first taller one.
    /// 4. Push current person onto the stack.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                result[i] += 1;
                if heights[i] <= heights[top] {
                    break;
                }
                stack.pop();
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
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        );
    }

    #[test]
    fn test_increasing() {
        assert_eq!(
            Solution::can_see_persons_count(vec![1, 2, 3]),
            vec![1, 1, 0]
        );
    }
}
