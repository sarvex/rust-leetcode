use std::collections::HashMap;

impl Solution {
    /// Counts subarrays summing to k using prefix sum frequency tracking.
    ///
    /// # Intuition
    /// If prefix_sum[j] - prefix_sum[i] == k, then subarray (i, j] sums to k.
    /// Track prefix sum frequencies in a hash map for O(1) lookups.
    ///
    /// # Approach
    /// 1. Maintain a running prefix sum.
    /// 2. For each sum, add the count of (sum - k) from the map.
    /// 3. Increment the current sum's count in the map.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::new();
        freq.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;
        for &x in &nums {
            sum += x;
            if let Some(&v) = freq.get(&(sum - k)) {
                count += v;
            }
            *freq.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn test_with_negatives() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
    }

    #[test]
    fn test_single_match() {
        assert_eq!(Solution::subarray_sum(vec![5], 5), 1);
    }

    #[test]
    fn test_negative_numbers() {
        // [-1, -1, 1] -> subarrays summing to 0: [-1, -1, 1, 1] not possible
        // Actually: [-1,-1,1] = -1, [-1,1] = 0, [1] = 1, [-1] = -1, etc
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
    }

    #[test]
    fn test_all_zeros() {
        // [0, 0, 0] with k=0: [0], [0], [0], [0,0], [0,0], [0,0,0] = 6 subarrays
        assert_eq!(Solution::subarray_sum(vec![0, 0, 0], 0), 6);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 100), 0);
    }

    #[test]
    fn test_entire_array() {
        // Sum of entire array equals k
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3, 4], 10), 1);
    }
}
