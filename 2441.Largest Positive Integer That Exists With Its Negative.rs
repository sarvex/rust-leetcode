use std::collections::HashSet;

impl Solution {
    /// Finds the largest positive integer k such that -k also exists in the array.
    ///
    /// # Intuition
    /// Collect all numbers into a set, then find the maximum positive value
    /// whose negation is also present.
    ///
    /// # Approach
    /// 1. Build a HashSet from all elements
    /// 2. Filter for positive values whose negation exists in the set
    /// 3. Return the maximum, or -1 if none found
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().copied().collect();
        set.iter()
            .filter(|x| **x > 0 && set.contains(&-**x))
            .copied()
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
    }

    #[test]
    fn test_no_pair() {
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::find_max_k(vec![1, -1]), 1);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(Solution::find_max_k(vec![1, 2, 3]), -1);
    }
}
