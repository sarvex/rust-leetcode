impl Solution {
    /// Returns the length of the longest almost-palindromic substring using DP.
    ///
    /// # Intuition
    /// A substring is almost-palindromic if it becomes a palindrome after removing exactly one
    /// character. This is equivalent to finding the longest substring that requires at most 1
    /// character removal to become a palindrome.
    ///
    /// # Approach
    /// Use dynamic programming to compute the minimum number of character removals needed to make
    /// each substring s[i..=j] a palindrome. Let dp[i][j] represent this minimum count.
    ///
    /// State transitions:
    /// - If s[i] == s[j]: dp[i][j] = dp[i+1][j-1] (match outer characters, solve inner substring)
    /// - If s[i] != s[j]: dp[i][j] = 1 + min(dp[i+1][j], dp[i][j-1]) (remove s[i] or s[j])
    ///
    /// A substring is almost-palindromic if dp[i][j] <= 1. We track the maximum length of such
    /// substrings.
    ///
    /// # Complexity
    /// - Time: O(n²) where n is the length of s. We fill an n×n DP table with constant work per cell.
    /// - Space: O(n²) for the DP table. Can be optimized to O(n) but n ≤ 2500 makes this acceptable.
    ///
    /// # Panics
    /// Panics if the input string is empty (though constraints guarantee length ≥ 2).
    pub fn longest_almost_palindromic_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        if n < 2 {
            return n as i32;
        }

        // dp[i][j] = minimum removals to make s[i..=j] a palindrome
        let mut dp = vec![vec![0i16; n]; n];
        let mut max_len = 1i32;

        // Fill DP table for substrings of length 2 to n
        for length in 2..=n {
            for i in 0..=n - length {
                let j = i + length - 1;

                if s[i] == s[j] {
                    // Characters match, inner substring determines the cost
                    dp[i][j] = if length == 2 { 0 } else { dp[i + 1][j - 1] };
                } else {
                    // Characters don't match, must remove one of them
                    dp[i][j] = 1 + dp[i + 1][j].min(dp[i][j - 1]);
                }

                // Check if this substring is almost-palindromic (≤ 1 removal needed)
                if dp[i][j] <= 1 {
                    max_len = max_len.max(length as i32);
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("abca".to_string()),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("abba".to_string()),
            4
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("zzabba".to_string()),
            5
        );
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("ab".to_string()),
            2
        );
        assert_eq!(
            Solution::longest_almost_palindromic_substring("aa".to_string()),
            2
        );
    }

    #[test]
    fn test_all_same_characters() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("aaaaa".to_string()),
            5
        );
    }

    #[test]
    fn test_no_long_palindrome() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("abcde".to_string()),
            2
        );
    }

    #[test]
    fn test_abcba_pattern() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("abcba".to_string()),
            5
        );
    }

    #[test]
    fn test_long_string_palindrome() {
        let s = "a".repeat(2500);
        assert_eq!(Solution::longest_almost_palindromic_substring(s), 2500);
    }

    #[test]
    fn test_almost_palindrome_at_end() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("xyzabba".to_string()),
            5
        );
    }

    #[test]
    fn test_almost_palindrome_at_start() {
        assert_eq!(
            Solution::longest_almost_palindromic_substring("abbcdef".to_string()),
            3
        );
    }
}
