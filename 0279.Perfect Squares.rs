impl Solution {
    /// Finds the minimum number of perfect squares summing to n using DP.
    ///
    /// # Intuition
    /// This is an unbounded knapsack problem where perfect squares are items
    /// and n is the target weight.
    ///
    /// # Approach
    /// 1. Create a 2D DP table: rows are perfect squares (1..sqrt(n)),
    ///    columns are target sums (0..n).
    /// 2. For each square, either skip it or use it (unbounded).
    /// 3. Take the minimum count.
    ///
    /// # Complexity
    /// - Time: O(n * sqrt(n))
    /// - Space: O(n * sqrt(n))
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let max_sq = (n as f64).sqrt() as usize;
        let mut dp = vec![vec![i32::MAX; n + 1]; max_sq + 1];
        dp[0][0] = 0;
        for i in 1..=max_sq {
            for j in 0..=n {
                dp[i][j] = dp[i - 1][j];
                if j >= i * i && dp[i][j - i * i] != i32::MAX {
                    dp[i][j] = dp[i][j].min(dp[i][j - i * i] + 1);
                }
            }
        }
        dp[max_sq][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_needs_three() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn thirteen_needs_two() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn perfect_square() {
        assert_eq!(Solution::num_squares(16), 1);
    }
}
