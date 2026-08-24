impl Solution {
    /// Greedy DP over prefix sums to find optimal score difference.
    ///
    /// # Intuition
    /// When a player picks the leftmost `x` stones, their score equals `prefix[x-1]`
    /// (the prefix sum up to index `x-1`). After the pick, a stone with that prefix
    /// sum value is placed back. The key insight is that a player choosing index `i`
    /// effectively scores `prefix[i]`. We define `dp[i]` as the maximum score
    /// difference achievable for the current player when the smallest choosable
    /// index is `i`. The recurrence is `dp[i] = max(prefix[i] - dp[i+1], dp[i+1])`,
    /// which we compute right-to-left, maintaining just the suffix maximum.
    ///
    /// # Approach
    /// 1. Compute prefix sums of the stones array.
    /// 2. Since the minimum pick is `x > 1`, the first choosable prefix sum index
    ///    is `1`. Iterate from `n-1` down to `1`.
    /// 3. Maintain a running value: `dp = max(dp, prefix[i] - dp)` simplified to
    ///    tracking the suffix max of `prefix[i] - dp[i+1]`. Actually, we iterate
    ///    right-to-left keeping `best = max(best, prefix[i])` isn't quite right —
    ///    the correct recurrence is `dp[i] = max(prefix[i] - dp[i+1], dp[i+1])`.
    ///    We just keep a single variable for `dp[i+1]` and update from right to left.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) (in-place prefix sum modification)
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix = stones;
        for i in 1..n {
            prefix[i] += prefix[i - 1];
        }

        // dp starts at the rightmost choice: if current player picks all stones,
        // they score prefix[n-1]. There's no further move, so dp[n-1] = prefix[n-1].
        let mut dp = prefix[n - 1];

        // Iterate from n-2 down to 1 (index 0 is not choosable since x > 1)
        for i in (1..n - 1).rev() {
            // Current player can pick index i (scoring prefix[i]) and opponent
            // then faces dp value, or skip index i (equivalent to dp[i+1]).
            dp = dp.max(prefix[i] - dp);
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]),
            13
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::stone_game_viii(vec![-10, -12]), -22);
    }

    #[test]
    fn test_two_positive() {
        assert_eq!(Solution::stone_game_viii(vec![1, 2]), 3);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::stone_game_viii(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(Solution::stone_game_viii(vec![1, 2, 3, 4, 5]), 15);
    }
}
