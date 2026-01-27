impl Solution {
    /// Longest palindromic subsequence DP to check k-removal palindrome.
    ///
    /// # Intuition
    /// A string is a valid k-palindrome if at most k characters can be removed
    /// to make it a palindrome. This is equivalent to checking whether the
    /// longest palindromic subsequence has length ≥ n − k. Early termination
    /// when the condition is met avoids unnecessary computation.
    ///
    /// # Approach
    /// 1. Build a 2D DP table where `dp[i][j]` = length of longest palindromic
    ///    subsequence in `s[i..=j]`
    /// 2. Fill bottom-up: if endpoints match, extend by 2; otherwise take max
    ///    of excluding either endpoint
    /// 3. Return true as soon as `dp[i][j] + k ≥ n` for any subproblem
    ///
    /// # Complexity
    /// - Time: O(n²) for the DP table
    /// - Space: O(n²) for the DP table
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0i32; n]; n];

        for i in 0..n {
            dp[i][i] = 1;
        }

        for i in (0..n.saturating_sub(1)).rev() {
            for j in i + 1..n {
                dp[i][j] = if s[i] == s[j] {
                    dp[i + 1][j - 1] + 2
                } else {
                    dp[i + 1][j].max(dp[i][j - 1])
                };

                if dp[i][j] + k >= n as i32 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_one_character() {
        assert!(Solution::is_valid_palindrome("abcdeca".to_string(), 2));
    }

    #[test]
    fn already_palindrome() {
        assert!(Solution::is_valid_palindrome("aba".to_string(), 0));
    }

    #[test]
    fn not_enough_removals() {
        assert!(!Solution::is_valid_palindrome("abcdef".to_string(), 1));
    }

    #[test]
    fn single_character() {
        assert!(Solution::is_valid_palindrome("a".to_string(), 0));
    }
}
