impl Solution {
    /// Dynamic programming table for longest palindromic substring.
    ///
    /// # Intuition
    /// A substring `s[i..=j]` is a palindrome if its outer characters match
    /// and the inner substring `s[i+1..=j-1]` is also a palindrome. Building
    /// this relation bottom-up captures all palindromic substrings.
    ///
    /// # Approach
    /// Create a 2D boolean DP table where `dp[start][end]` indicates whether
    /// `s[start..=end]` is a palindrome. Iterate `end` from 1 to n-1 and
    /// `start` from 0 to `end`. A cell is true when endpoints match and the
    /// gap is less than 2 or the inner substring is a palindrome. Track the
    /// longest palindrome found.
    ///
    /// # Complexity
    /// - Time: O(n^2) — filling the DP table
    /// - Space: O(n^2) — the DP table
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut dp = vec![vec![false; n]; n];
        let (mut best_start, mut best_len) = (0, 1);

        for end in 1..n {
            for start in 0..=end {
                if bytes[start] == bytes[end] && (end - start < 2 || dp[start + 1][end - 1]) {
                    dp[start][end] = true;
                    if end - start + 1 > best_len {
                        best_start = start;
                        best_len = end - start + 1;
                    }
                }
            }
        }

        s[best_start..best_start + best_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_length_palindrome() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn even_length_palindrome() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn entire_string_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("racecar".to_string()),
            "racecar"
        );
    }

    #[test]
    fn two_characters() {
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a");
    }
}
