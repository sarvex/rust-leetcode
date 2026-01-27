impl Solution {
    /// Predicts if player 1 can win using interval DP on score difference.
    ///
    /// # Intuition
    /// dp[i][j] represents the maximum score difference the current player can
    /// achieve from nums[i..=j]. If dp[0][n-1] >= 0, player 1 wins.
    ///
    /// # Approach
    /// 1. Initialize dp[i][i] = nums[i] (single element, must take it).
    /// 2. Fill diagonally: dp[i][j] = max(nums[i] - dp[i+1][j], nums[j] - dp[i][j-1]).
    /// 3. Check if dp[0][n-1] >= 0.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = nums[i];
        }
        for i in (0..n.saturating_sub(1)).rev() {
            for j in i + 1..n {
                dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
            }
        }
        dp[0][n - 1] >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player1_loses() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
    }

    #[test]
    fn test_player1_wins() {
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }

    #[test]
    fn test_single() {
        assert!(Solution::predict_the_winner(vec![10]));
    }
}
