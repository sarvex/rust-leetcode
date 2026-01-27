impl Solution {
    /// Counts ways to roll n dice with k faces to reach a target sum.
    ///
    /// # Intuition
    /// DP where `dp[i][j]` is the number of ways to get sum j using i dice.
    /// Each die face adds 1..=k to the previous sum.
    ///
    /// # Approach
    /// Base case: `dp[0][0] = 1`. For each die, for each reachable sum,
    /// add contributions from all face values.
    ///
    /// # Complexity
    /// - Time: O(n * target * k)
    /// - Space: O(n * target)
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let mut dp = vec![vec![0i32; target + 1]; n + 1];
        dp[0][0] = 1;

        for i in 1..=n {
            for j in 1..=target.min(i * k) {
                for face in 1..=j.min(k) {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - face]) % MOD;
                }
            }
        }

        dp[n][target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
    }

    #[test]
    fn test_two_dice() {
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
    }

    #[test]
    fn test_larger() {
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }
}
