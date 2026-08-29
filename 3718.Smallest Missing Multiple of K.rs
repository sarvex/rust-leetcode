use std::collections::HashSet;

impl Solution {
    /// Finds the smallest positive multiple of k missing from nums.
    ///
    /// # Intuition
    /// Collect all multiples of k present in nums into a set, then iterate
    /// through k, 2k, 3k, ... until we find one not in the set.
    ///
    /// # Approach
    /// 1. Filter nums to keep only multiples of k and store them in a HashSet.
    /// 2. Starting from k, check each successive multiple until one is absent.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n is nums.len() and m is the answer / k
    /// - Space: O(n)
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let multiples: HashSet<i32> = nums.into_iter().filter(|x| x % k == 0).collect();
        let mut m = k;
        while multiples.contains(&m) {
            m += k;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
    }

    #[test]
    fn test_k_equals_1() {
        assert_eq!(Solution::missing_multiple(vec![1, 2, 3, 4, 5], 1), 6);
    }

    #[test]
    fn test_no_multiples_present() {
        assert_eq!(Solution::missing_multiple(vec![1, 3, 7, 11], 5), 5);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::missing_multiple(vec![4], 4), 8);
    }
}
