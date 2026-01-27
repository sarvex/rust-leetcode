impl Solution {
    /// Returns the maximum of the count of positive and negative integers.
    ///
    /// # Intuition
    /// The array is sorted, so binary search finds the boundary between negatives,
    /// zeros, and positives efficiently.
    ///
    /// # Approach
    /// 1. Binary search for the first index >= 0 (negative count)
    /// 2. Binary search for the first index >= 1 (positive count = n - that index)
    /// 3. Return the max of both counts
    ///
    /// # Complexity
    /// - Time: O(log n) — two binary searches
    /// - Space: O(1)
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let neg_count = nums.partition_point(|&x| x < 0) as i32;
        let pos_count = (nums.len() - nums.partition_point(|&x| x < 1)) as i32;
        neg_count.max(pos_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed() {
        assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    }

    #[test]
    fn test_with_zeros() {
        assert_eq!(Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(Solution::maximum_count(vec![5, 20, 66, 1314]), 4);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::maximum_count(vec![0, 0, 0]), 0);
    }
}
