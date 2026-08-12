use std::collections::HashMap;

impl Solution {
    /// Sliding window to find longest subarray with element frequency ≤ k.
    ///
    /// # Intuition
    /// A sliding window where we expand the right boundary and shrink the left
    /// whenever a new element pushes its frequency above k. The window always
    /// represents a valid "good" subarray.
    ///
    /// # Approach
    /// Maintain a frequency map and two pointers `left`/`right`. For each `right`,
    /// increment the count of `nums[right]`. If that count exceeds `k`, advance
    /// `left` (decrementing counts) until the count is back to k. Track the
    /// maximum window size seen.
    ///
    /// # Complexity
    /// - Time: O(n) — each element enters and leaves the window at most once
    /// - Space: O(n) — frequency map holds at most n distinct elements
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::with_capacity(nums.len());
        let mut left = 0;

        nums.iter()
            .enumerate()
            .map(|(right, &num)| {
                *freq.entry(num).or_insert(0) += 1;
                while freq[&num] > k {
                    *freq.get_mut(&nums[left]).unwrap() -= 1;
                    left += 1;
                }
                (right - left + 1) as i32
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [1,2,3,1,2,3] has each value at most twice
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2),
            6
        );
    }

    #[test]
    fn test_example_2() {
        // k=1: any element can appear only once; best window is 2
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1),
            2
        );
    }

    #[test]
    fn test_example_3() {
        // All same element, k=4: window of 4
        assert_eq!(
            Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4),
            4
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_subarray_length(vec![42], 1), 1);
    }

    #[test]
    fn test_all_distinct() {
        // Every element unique, any k >= 1 means whole array is good
        assert_eq!(Solution::max_subarray_length(vec![1, 2, 3, 4, 5], 1), 5);
    }

    #[test]
    fn test_k_equals_length() {
        // k large enough: entire array is always good
        assert_eq!(Solution::max_subarray_length(vec![1, 1, 1, 1], 4), 4);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(Solution::max_subarray_length(vec![1, 2, 1, 2, 1], 3), 5);
    }
}
