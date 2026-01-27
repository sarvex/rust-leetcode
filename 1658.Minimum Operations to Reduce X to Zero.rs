use std::collections::HashMap;

impl Solution {
    /// Find longest middle subarray with sum equal to total - x.
    ///
    /// # Intuition
    /// Removing elements from both ends to reach sum x is equivalent to
    /// finding the longest contiguous subarray with sum `total - x`. The
    /// answer is `n - max_subarray_length`.
    ///
    /// # Approach
    /// 1. Compute target = total_sum - x
    /// 2. Use prefix sum + hash map to find longest subarray summing to target
    /// 3. Return n - max_length, or -1 if not found
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the hash map
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;
        let mut first_seen: HashMap<i32, i32> = HashMap::new();
        first_seen.insert(0, -1);

        let mut max_len = -1;
        let mut prefix = 0;

        for (i, &v) in nums.iter().enumerate() {
            prefix += v;
            first_seen.entry(prefix).or_insert(i as i32);
            if let Some(&j) = first_seen.get(&(prefix - target)) {
                max_len = max_len.max(i as i32 - j);
            }
        }

        if max_len == -1 {
            -1
        } else {
            nums.len() as i32 - max_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_both_ends() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn all_elements() {
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    }

    #[test]
    fn take_all() {
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }
}
