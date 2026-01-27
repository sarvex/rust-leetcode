impl Solution {
    /// Finds the longest common subsequence length using 2D DP.
    ///
    /// # Intuition
    /// If characters match, extend the LCS from the diagonal. Otherwise
    /// take the max from excluding one character from either string.
    ///
    /// # Approach
    /// `dp[i][j]` = LCS length of `text1[0..i]` and `text2[0..j]`. If
    /// characters match, `dp[i][j] = dp[i-1][j-1] + 1`; else max of
    /// `dp[i-1][j]` and `dp[i][j-1]`.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n)
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (a, b) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    dp[i - 1][j].max(dp[i][j - 1])
                };
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn test_identical() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn test_no_common() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
