impl Solution {
    /// Mathematical simulation tracking position transformations
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
    /// For Operation 1 (left-to-right): We delete positions 2, 4, 6... keeping positions
    /// 1, 3, 5... The first element survives.
    ///
    /// For Operation 2 (right-to-left): We delete the 2nd from right, 4th from right, etc.
    /// The leftmost survives if count is odd.
    ///
    /// # Complexity
    /// - Time: O(log n) - count halves each iteration
    /// - Space: O(1) - only tracking a few variables
    pub fn last_integer(n: i64) -> i64 {
        let mut left = 1i64;
        let mut step = 1i64;
        let mut count = n;
        let mut from_left = true;

        while count > 1 {
            if from_left {
                // From left: delete 2nd, 4th, 6th... keep 1st, 3rd, 5th...
                // First element survives
            } else {
                // From right: delete 2nd from right, 4th from right...
                // Leftmost survives only if count is odd
                if count % 2 == 0 {
                    left += step;
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
    fn test_example_1() {
        assert_eq!(Solution::last_remaining(8), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::last_remaining(5), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::last_remaining(1), 1);
    }

    #[test]
    fn test_small_values() {
        assert_eq!(Solution::last_remaining(2), 1);
        assert_eq!(Solution::last_remaining(3), 3);
        assert_eq!(Solution::last_remaining(4), 3);
    }

    #[test]
    fn test_large_value() {
        let result = Solution::last_remaining(1_000_000_000_000_000);
        assert!(result > 0);
    }
}
