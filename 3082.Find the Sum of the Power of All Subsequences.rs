const MOD: i64 = 1_000_000_007;

impl Solution {
    /// Weighted subsequence DP via supersequence counting.
    ///
    /// # Intuition
    /// For any subsequence `T` with sum `k`, every element not in `T` can be either kept or
    /// skipped in the outer subsequence, producing `2^(n - |T|)` supersequences. Summing power
    /// over all subsequences is therefore the sum of these weights over all `T` with sum `k`.
    ///
    /// # Approach
    /// Maintain `dp[sum] = total weight` for subsequences of the processed prefix with sum `sum`.
    /// When processing `value`:
    /// - Excluding `value` from `T` doubles the weight (`* 2`), because it can be in or out of
    ///   the outer subsequence.
    /// - Including `value` adds the previous weight to `sum + value`.
    /// Use a fresh `next` array each step and apply modulo arithmetic.
    ///
    /// # Complexity
    /// - Time: O(n * k)
    /// - Space: O(k)
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        let target = k as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for value in nums {
            let val = value as usize;
            let mut next = vec![0i64; target + 1];
            for sum in 0..=target {
                let doubled = (dp[sum] * 2) % MOD;
                next[sum] = (next[sum] + doubled) % MOD;
                if sum + val <= target {
                    next[sum + val] = (next[sum + val] + dp[sum]) % MOD;
                }
            }
            dp = next;
        }

        dp[target] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 3), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::sum_of_power(vec![2, 3, 3], 5), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 7), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::sum_of_power(vec![5], 5), 1);
    }

    #[test]
    fn test_multiple_singleton_hits() {
        assert_eq!(Solution::sum_of_power(vec![1, 1, 1], 1), 12);
    }
}
