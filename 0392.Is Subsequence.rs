impl Solution {
    /// Checks if `s` is a subsequence of `t` using a two-pointer scan.
    ///
    /// # Intuition
    /// Greedily match each character of `s` against `t` from left to right.
    /// If every character finds a match, `s` is a subsequence.
    ///
    /// # Approach
    /// 1. Iterate over `t` bytes with a mutable pointer into `s`.
    /// 2. Advance the `s` pointer whenever a character matches.
    /// 3. Return true if the pointer reaches the end of `s`.
    ///
    /// # Complexity
    /// - Time: O(n) where n = t.len()
    /// - Space: O(1)
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let mut idx = 0;
        for &b in t.as_bytes() {
            if idx < s.len() && s[idx] == b {
                idx += 1;
            }
        }
        idx == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsequence() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_not_subsequence() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_empty_s() {
        assert!(Solution::is_subsequence(String::new(), "abc".to_string()));
    }

    #[test]
    fn test_both_empty() {
        assert!(Solution::is_subsequence(String::new(), String::new()));
    }

    #[test]
    fn test_s_longer_than_t() {
        assert!(!Solution::is_subsequence(
            "abcd".to_string(),
            "abc".to_string()
        ));
    }

    #[test]
    fn test_equal_strings() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "abc".to_string()
        ));
    }

    #[test]
    fn test_single_char_match() {
        assert!(Solution::is_subsequence("b".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_single_char_no_match() {
        assert!(!Solution::is_subsequence("z".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_consecutive_chars() {
        assert!(Solution::is_subsequence(
            "ace".to_string(),
            "abcde".to_string()
        ));
    }
}
