impl Solution {
    /// Mathematical simulation tracking position transformations.
    ///
    /// # Intuition
    /// Instead of simulating the actual deletion process (which would be O(n) and
    /// infeasible for n up to 10^15), we track how the leftmost element and step
    /// size change after each operation.
    ///
    /// # Approach
    /// After each deletion operation, the remaining sequence is an arithmetic
    /// progression. We track:
    /// - `left`: the current leftmost value
    /// - `step`: the difference between consecutive remaining elements
    /// - `count`: how many elements remain
    ///
    /// For Operation 1 (left-to-right): delete positions 2, 4, 6... keeping 1, 3, 5...
    /// For Operation 2 (right-to-left): delete 2nd from right, 4th from right, etc.
    /// The leftmost survives if count is odd.
    ///
    /// # Complexity
    /// - Time: O(log n) - count halves each iteration
    /// - Space: O(1)
    pub fn last_integer(n: i64) -> i64 {
        let mut left = 1i64;
        let mut step = 1i64;
        let mut count = n;
        let mut from_left = true;

        while count > 1 {
            match from_left {
                true => {}
                false => {
                    if count % 2 == 0 {
                        left += step;
                    }
                }
            }
            step *= 2;
            count = (count + 1) / 2;
            from_left = !from_left;
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_elements() {
        assert_eq!(Solution::last_integer(8), 3);
    }

    #[test]
    fn test_five_elements() {
        assert_eq!(Solution::last_integer(5), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::last_integer(1), 1);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::last_integer(2), 1);
    }

    #[test]
    fn test_three_elements() {
        assert_eq!(Solution::last_integer(3), 3);
    }

    #[test]
    fn test_four_elements() {
        assert_eq!(Solution::last_integer(4), 3);
    }

    #[test]
    fn test_large_value() {
        let result = Solution::last_integer(1_000_000_000_000_000);
        assert!(result > 0);
    }
}
