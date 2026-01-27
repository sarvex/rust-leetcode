impl Solution {
    /// Counts operations to make all elements zero by subtracting minimums.
    ///
    /// # Intuition
    /// Each operation removes the current minimum from all nonzero elements. The
    /// number of operations equals the count of distinct nonzero values.
    ///
    /// # Approach
    /// Collect into a HashSet and subtract 1 if zero is present.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let unique: HashSet<&i32> = nums.iter().collect();
        (unique.len() - unique.contains(&0) as usize) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::minimum_operations(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_all_same_nonzero() {
        assert_eq!(Solution::minimum_operations(vec![3, 3, 3]), 1);
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3]), 3);
    }
}
