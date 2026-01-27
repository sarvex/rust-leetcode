impl Solution {
    /// Finds minimum cost to split an array into subarrays.
    ///
    /// # Intuition
    /// This is a partition DP problem where each subarray's cost depends on
    /// elements that appear more than once (the "trimmed" version).
    ///
    /// # Approach
    /// - `dp[i]` = minimum cost to split `nums[0..i]`
    /// - For each position i, try all possible last subarrays ending at index i-1
    /// - Incrementally compute the trimmed length by tracking frequencies backward
    /// - Trimmed length = sum of frequencies for elements appearing >= 2 times
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n) for the DP array and frequency counter
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;

        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let mut freq = vec![0i32; n];
            let mut trimmed_len = 0i64;

            for j in (0..i).rev() {
                let val = nums[j] as usize;
                freq[val] += 1;

                match freq[val] {
                    2 => trimmed_len += 2,
                    c if c > 2 => trimmed_len += 1,
                    _ => {}
                }

                dp[i] = dp[i].min(dp[j].saturating_add(k + trimmed_len));
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 2), 6);
    }

    #[test]
    fn test_large_k_single_subarray() {
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 5), 10);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_cost(vec![0], 1), 1);
    }

    #[test]
    fn test_all_same_elements() {
        assert_eq!(Solution::min_cost(vec![0, 0, 0], 1), 4);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(Solution::min_cost(vec![0, 1, 2], 2), 2);
    }

    #[test]
    fn test_large_k_value() {
        assert_eq!(
            Solution::min_cost(vec![0, 1, 0, 1], 1_000_000_000),
            1_000_000_004
        );
    }
}
