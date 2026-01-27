use std::collections::HashSet;

impl Solution {
    /// Finds the duplicated and missing numbers using sum arithmetic.
    ///
    /// # Intuition
    /// The difference between the actual sum and the unique-element sum gives
    /// the duplicate. The difference between the expected sum and the unique
    /// sum gives the missing number.
    ///
    /// # Approach
    /// 1. Compute expected sum = n(n+1)/2.
    /// 2. Compute unique sum via HashSet.
    /// 3. Duplicate = actual_sum - unique_sum; missing = expected_sum - unique_sum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let expected = n * (n + 1) / 2;
        let actual: i32 = nums.iter().sum();
        let unique: i32 = nums.into_iter().collect::<HashSet<_>>().iter().sum();
        vec![actual - unique, expected - unique]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn test_first_missing() {
        assert_eq!(Solution::find_error_nums(vec![2, 2]), vec![2, 1]);
    }
}
