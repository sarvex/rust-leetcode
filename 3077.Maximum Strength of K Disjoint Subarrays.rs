impl Solution {
    /// In-place DP by subarray count with fixed coefficients.
    ///
    /// # Intuition
    /// The j-th chosen subarray always contributes the same coefficient
    /// `(k - j + 1) * (-1)^(j + 1)`, so we can use the classic
    /// "max sum of j disjoint subarrays" recurrence with weighted sums.
    ///
    /// # Approach
    /// Precompute coefficients `coeff[j]` for the j-th subarray (1-indexed).
    /// Maintain:
    /// - `dp[j]`: best strength using exactly `j` subarrays in the processed prefix.
    /// - `best[j]`: best strength where the j-th subarray ends at the current element.
    /// For each value, update `j` in descending order to preserve previous states:
    /// `best[j] = max(best[j], dp[j - 1]) + coeff[j] * value`
    /// `dp[j] = max(dp[j], best[j])`
    ///
    /// # Complexity
    /// - Time: O(n * k)
    /// - Space: O(k)
    pub fn maximum_strength(nums: Vec<i32>, k: i32) -> i64 {
        let groups = k as usize;
        let mut coefficients = vec![0i64; groups];
        for (idx, coeff) in coefficients.iter_mut().enumerate() {
            let magnitude = (groups - idx) as i64;
            *coeff = if (idx + 1) % 2 == 1 {
                magnitude
            } else {
                -magnitude
            };
        }

        const NEG_INF: i64 = i64::MIN / 4;
        let mut dp = vec![NEG_INF; groups + 1];
        let mut best = vec![NEG_INF; groups + 1];
        dp[0] = 0;

        for (index, &value) in nums.iter().enumerate() {
            let upper = groups.min(index + 1);
            let x = value as i64;
            for j in (1..=upper).rev() {
                let delta = coefficients[j - 1] * x;
                let extend = if best[j] == NEG_INF {
                    NEG_INF
                } else {
                    best[j] + delta
                };
                let start_new = if dp[j - 1] == NEG_INF {
                    NEG_INF
                } else {
                    dp[j - 1] + delta
                };
                let new_best = extend.max(start_new);
                best[j] = new_best;
                dp[j] = dp[j].max(new_best);
            }
        }

        dp[groups]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::maximum_strength(vec![1, 2, 3, -1, 2], 3), 22);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::maximum_strength(vec![12, -2, -2, -2, -2], 5), 64);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::maximum_strength(vec![-1, -2, -3], 1), -1);
    }

    #[test]
    fn test_k_equals_n() {
        assert_eq!(Solution::maximum_strength(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn test_forced_singletons() {
        assert_eq!(Solution::maximum_strength(vec![1, -5, 2], 3), 15);
    }
}
