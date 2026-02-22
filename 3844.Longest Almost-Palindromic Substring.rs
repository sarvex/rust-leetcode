impl Solution {
    /// Returns the length of the longest almost-palindromic substring using optimized DP.
    ///
    /// # Intuition
    /// A substring is almost-palindromic if at most one character removal makes it a palindrome.
    /// We use DP to compute minimum removals but optimize space to O(n).
    ///
    /// # Approach
    /// dp[j] represents the minimum removals needed for substring s[i..=j] for current i.
    /// We iterate i from n-1 down to 0, updating dp[j] for j >= i.
    ///
    /// State transitions (for i < j):
    /// - If s[i] == s[j]: dp[j] = prev_diag (dp[i+1][j-1] from previous iteration)
    /// - If s[i] != s[j]: dp[j] = 1 + min(dp[j], dp[j-1])
    ///   where dp[j] was just updated to dp[i+1][j] and dp[j-1] is dp[i][j-1]
    ///
    /// We track the maximum length where removals <= 1.
    ///
    /// # Complexity
    /// - Time: O(n²) where n is the length of s.
    /// - Space: O(n) for the 1D DP array, down from O(n²) in the naive approach.
    pub fn almost_palindromic(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        if n < 2 {
            return n as i32;
        }

        // dp[j] = minimum removals to make s[current_i..=j] a palindrome
        let mut dp = vec![0i16; n];
        let mut max_len = 1i32;

        // Iterate i from n-1 down to 0
        for i in (0..n).rev() {
            let mut prev_diag = 0i16; // This will hold dp[i+1][j-1]

            for j in i + 1..n {
                let temp = dp[j]; // Save before we overwrite (this is dp[i+1][j])

                if s[i] == s[j] {
                    // dp[i][j] = dp[i+1][j-1] = prev_diag
                    dp[j] = prev_diag;
                } else {
                    // dp[i][j] = 1 + min(dp[i+1][j], dp[i][j-1])
                    // dp[i+1][j] = temp (saved before overwrite)
                    // dp[i][j-1] = dp[j-1] (already computed for current i)
                    dp[j] = 1 + temp.min(dp[j - 1]);
                }

                // Update prev_diag for next iteration (dp[i+1][j] becomes dp[i+1][j-1] for next j)
                prev_diag = temp;

                // Check if almost-palindromic
                if dp[j] <= 1 {
                    max_len = max_len.max((j - i + 1) as i32);
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
        assert_eq!(Solution::almost_palindromic("abca".to_string()), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::almost_palindromic("abba".to_string()), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::almost_palindromic("zzabba".to_string()), 5);
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::almost_palindromic("ab".to_string()), 2);
        assert_eq!(Solution::almost_palindromic("aa".to_string()), 2);
    }

    #[test]
    fn test_all_same_characters() {
        assert_eq!(Solution::almost_palindromic("aaaaa".to_string()), 5);
    }

    #[test]
    fn test_no_long_palindrome() {
        assert_eq!(Solution::almost_palindromic("abcde".to_string()), 2);
    }

    #[test]
    fn test_abcba_pattern() {
        assert_eq!(Solution::almost_palindromic("abcba".to_string()), 5);
    }

    #[test]
    fn test_long_string_palindrome() {
        let s = "a".repeat(2500);
        assert_eq!(Solution::almost_palindromic(s), 2500);
    }

    #[test]
    fn test_almost_palindrome_at_end() {
        assert_eq!(Solution::almost_palindromic("xyzabba".to_string()), 5);
    }

    #[test]
    fn test_almost_palindrome_at_start() {
        assert_eq!(Solution::almost_palindromic("abbcdef".to_string()), 3);
    }
}
