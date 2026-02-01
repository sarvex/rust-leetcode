use std::collections::HashMap;

impl Solution {
    /// DP by last value and number of transitions; compress values and maintain
    /// best/second-best per transition count for O(1) "best other value" lookup.
    ///
    /// # Intuition
    /// A good subsequence has at most `k` indices where consecutive elements differ. We want the
    /// longest such subsequence. For each position we either extend from the same value (no new
    /// transition) or from a different value (one new transition).
    ///
    /// # Approach
    /// - Compress values to indices so we can index `dp` by value.
    /// - `dp[v][t]` = max length of a good subsequence ending with value `v` using at most `t`
    ///   transitions.
    /// - For each `nums[i] = v`: new length = 1 + max(dp[v][t], best_other[t-1]). Track
    ///   best and second-best length per `t` (and which value gave best) so "best other than v"
    ///   is O(1).
    /// - Answer is max over all (value, t) of dp[v][t].
    ///
    /// # Complexity
    /// - Time: O(n * k)
    /// - Space: O(unique_vals * k)
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut sorted: Vec<i32> = nums.iter().copied().collect();
        sorted.sort_unstable();
        sorted.dedup();
        let val_to_idx: HashMap<i32, usize> = sorted.iter().copied().enumerate().map(|(i, v)| (v, i)).collect();
        let n_vals = sorted.len();

        let mut dp = vec![vec![0i32; k + 1]; n_vals];
        let mut best_len = vec![0i32; k + 1];
        let mut best_val = vec![n_vals; k + 1];
        let mut second_len = vec![0i32; k + 1];

        for &v in &nums {
            let idx = val_to_idx[&v];
            let mut new_vals = vec![0i32; k + 1];
            for t in 0..=k {
                let other = if t >= 1 {
                    if best_val[t - 1] == idx {
                        second_len[t - 1]
                    } else {
                        best_len[t - 1]
                    }
                } else {
                    0
                };
                new_vals[t] = 1 + dp[idx][t].max(other);
            }
            for t in 0..=k {
                let new_val = new_vals[t];
                dp[idx][t] = dp[idx][t].max(new_val);
                if new_val >= best_len[t] {
                    second_len[t] = best_len[t];
                    best_len[t] = new_val;
                    best_val[t] = idx;
                } else if new_val > second_len[t] {
                    second_len[t] = new_val;
                }
            }
        }

        *best_len.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 1, 1, 3], 2), 4);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5, 1], 0), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_length(vec![1], 0), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::maximum_length(vec![7, 7, 7, 7], 0), 4);
    }

    #[test]
    fn test_k_large() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 10), 5);
    }
}
