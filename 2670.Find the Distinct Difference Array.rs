use std::collections::HashSet;

impl Solution {
    /// Compute distinct difference array using prefix and suffix distinct counts.
    ///
    /// # Intuition
    /// Precompute suffix distinct counts, then scan left to right tracking
    /// prefix distinct counts. The difference at each position is prefix - suffix.
    ///
    /// # Approach
    /// 1. Build suffix distinct counts from right to left using a HashSet
    /// 2. Scan left to right, maintaining a prefix HashSet
    /// 3. Output prefix_count - suffix_count at each index
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut suffix_distinct = vec![0usize; n + 1];
        let mut seen = HashSet::new();

        for i in (0..n).rev() {
            seen.insert(nums[i]);
            suffix_distinct[i] = seen.len();
        }

        seen.clear();
        let mut ans = Vec::with_capacity(n);

        for i in 0..n {
            seen.insert(nums[i]);
            ans.push((seen.len() - suffix_distinct[i + 1]) as i32);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
            vec![-3, -1, 1, 3, 5]
        );
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
            vec![-2, -1, 0, 2, 3]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::distinct_difference_array(vec![1]), vec![1]);
    }
}
