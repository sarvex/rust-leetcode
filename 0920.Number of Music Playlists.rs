impl Solution {
    /// Counts distinct playlists of length goal from n songs with gap constraint k.
    ///
    /// # Intuition
    /// DP where `dp[i][j]` is the number of playlists of length i using exactly
    /// j distinct songs. At each step, either add a new song or replay one
    /// allowed by the gap constraint.
    ///
    /// # Approach
    /// For a new song: `dp[i-1][j-1] * (n - j + 1)`. For a replayed song
    /// (only if j > k): `dp[i-1][j] * (j - k)`.
    ///
    /// # Complexity
    /// - Time: O(goal * n)
    /// - Space: O(goal * n) for the DP table
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (n, goal, k) = (n as usize, goal as usize, k as usize);
        let mut dp = vec![vec![0i64; n + 1]; goal + 1];
        dp[0][0] = 1;

        for i in 1..=goal {
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j - 1] * (n - j + 1) as i64 % MOD;
                if j > k {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j] * (j - k) as i64) % MOD;
                }
            }
        }

        dp[goal][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
    }

    #[test]
    fn test_with_repeats() {
        assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
    }

    #[test]
    fn test_larger() {
        assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
    }
}
