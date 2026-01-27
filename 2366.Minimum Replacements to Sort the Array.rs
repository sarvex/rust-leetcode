impl Solution {
    /// Greedy right-to-left traversal with ceiling division for optimal splits.
    ///
    /// # Intuition
    /// The rightmost element should remain unchanged since splitting only produces
    /// smaller values. By traversing right to left, we determine the minimum
    /// operations while maintaining the largest possible upper bound at each position.
    ///
    /// # Approach
    /// 1. Traverse from right to left, tracking the maximum allowed value
    /// 2. For elements exceeding the bound, compute minimum splits via ceiling division
    /// 3. Update the bound to the smallest resulting piece: `num / parts`
    /// 4. Accumulate `(parts - 1)` operations per split
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut total_operations: i64 = 0;

        let mut upper_bound = match nums.last() {
            Some(&value) => value,
            None => return 0,
        };

        for &current_value in nums.iter().rev().skip(1) {
            if current_value <= upper_bound {
                upper_bound = current_value;
                continue;
            }

            let num_parts = Self::ceiling_div(current_value, upper_bound);
            total_operations += i64::from(num_parts - 1);
            upper_bound = current_value / num_parts;
        }

        total_operations
    }

    /// Computes ceiling division: ⌈dividend / divisor⌉
    #[inline]
    const fn ceiling_div(dividend: i32, divisor: i32) -> i32 {
        (dividend + divisor - 1) / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_requires_splits() {
        assert_eq!(Solution::minimum_replacement(vec![3, 9, 3]), 2);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(Solution::minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_replacement(vec![42]), 0);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(Solution::minimum_replacement(vec![]), 0);
    }

    #[test]
    fn test_two_elements_requiring_split() {
        assert_eq!(Solution::minimum_replacement(vec![10, 3]), 3);
    }

    #[test]
    fn test_descending_array() {
        assert_eq!(Solution::minimum_replacement(vec![5, 4, 3, 2, 1]), 10);
    }

    #[test]
    fn test_all_equal_elements() {
        assert_eq!(Solution::minimum_replacement(vec![7, 7, 7, 7]), 0);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(
            Solution::minimum_replacement(vec![1_000_000_000, 1]),
            999_999_999
        );
    }

    #[test]
    fn test_ceiling_div_exact() {
        assert_eq!(Solution::ceiling_div(10, 5), 2);
    }

    #[test]
    fn test_ceiling_div_with_remainder() {
        assert_eq!(Solution::ceiling_div(11, 5), 3);
    }
}
