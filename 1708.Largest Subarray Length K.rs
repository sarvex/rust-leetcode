impl Solution {
    /// Finds the lexicographically largest subarray of length k.
    ///
    /// # Intuition
    /// Since all elements are distinct, the largest subarray starts at the position
    /// of the maximum element among the first `n - k + 1` elements.
    ///
    /// # Approach
    /// 1. Find the index of the maximum value in `nums[0..=n-k]`.
    /// 2. Return the slice `nums[idx..idx+k]`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(k) for the result
    pub fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let best = (0..=nums.len() - k).max_by_key(|&i| nums[i]).unwrap();
        nums[best..best + k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::largest_subarray(vec![1, 4, 5, 2, 3], 3),
            vec![5, 2, 3]
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::largest_subarray(vec![1, 4, 5, 2, 3], 4),
            vec![4, 5, 2, 3]
        );
    }

    #[test]
    fn test_full_array() {
        assert_eq!(Solution::largest_subarray(vec![3, 2, 1], 3), vec![3, 2, 1]);
    }
}
