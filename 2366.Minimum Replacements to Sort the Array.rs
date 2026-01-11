impl Solution {
    /// Greedy traversal from right to left with ceiling division for optimal splits.
    ///
    /// # Intuition
    /// The key insight is that the rightmost element should remain unchanged since
    /// any split only produces smaller values. By traversing from right to left,
    /// we can determine the minimum number of operations needed while maintaining
    /// the largest possible upper bound at each position.
    ///
    /// # Approach
    /// 1. Traverse the array from right to left, tracking the maximum allowed value
    /// 2. For each element greater than the current bound, calculate minimum splits
    /// 3. Use ceiling division to find the optimal number of parts: `(num + bound - 1) / bound`
    /// 4. After splitting, update the bound to the smallest resulting piece: `num / parts`
    /// 5. Accumulate `(parts - 1)` operations for each split
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of the input array
    /// - Space: O(1) constant extra space
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

    /// Computes ceiling division without overflow: ⌈dividend / divisor⌉
    #[inline]
    const fn ceiling_div(dividend: i32, divisor: i32) -> i32 {
        (dividend + divisor - 1) / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        expected: i64,
    }

    #[test]
    fn test_example_case_requires_splits() {
        let test = TestCase {
            nums: vec![3, 9, 3],
            expected: 2,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_already_sorted_array() {
        let test = TestCase {
            nums: vec![1, 2, 3, 4, 5],
            expected: 0,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_single_element() {
        let test = TestCase {
            nums: vec![42],
            expected: 0,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_empty_array() {
        let test = TestCase {
            nums: vec![],
            expected: 0,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_two_elements_requiring_split() {
        let test = TestCase {
            nums: vec![10, 3],
            expected: 3,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_descending_array() {
        let test = TestCase {
            nums: vec![5, 4, 3, 2, 1],
            expected: 10,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_all_equal_elements() {
        let test = TestCase {
            nums: vec![7, 7, 7, 7],
            expected: 0,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
    }

    #[test]
    fn test_large_values() {
        let test = TestCase {
            nums: vec![1_000_000_000, 1],
            expected: 999_999_999,
        };
        assert_eq!(Solution::minimum_replacement(test.nums), test.expected);
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
