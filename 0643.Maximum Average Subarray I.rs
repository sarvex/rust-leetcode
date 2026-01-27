impl Solution {
    /// Finds the maximum average of a subarray of length k using a sliding window.
    ///
    /// # Intuition
    /// Compute the sum of the first k elements, then slide the window by
    /// adding the next element and removing the oldest.
    ///
    /// # Approach
    /// 1. Compute the initial window sum.
    /// 2. Slide the window, tracking the maximum sum.
    /// 3. Return max_sum / k.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = nums.iter().take(k).sum();
        let mut best = sum;
        for i in k..nums.len() {
            sum += nums[i] - nums[i - k];
            best = best.max(sum);
        }
        best as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
        assert!((result - 12.75).abs() < 1e-5);
    }

    #[test]
    fn test_single() {
        assert!((Solution::find_max_average(vec![5], 1) - 5.0).abs() < 1e-5);
    }
}
