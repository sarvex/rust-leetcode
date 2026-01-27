impl Solution {
    /// Checks that no 'a' appears after a 'b' by detecting the "ba" pattern.
    ///
    /// # Intuition
    /// If the string contains "ba" as a substring, then at least one 'a'
    /// appears after a 'b', violating the requirement.
    ///
    /// # Approach
    /// 1. Check whether the substring "ba" exists in the input.
    /// 2. Return true if it does not.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check_string(s: String) -> bool {
        !s.as_bytes().windows(2).any(|w| w == b"ba")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ordering() {
        assert!(Solution::check_string("aaabbb".to_string()));
    }

    #[test]
    fn test_invalid_ordering() {
        assert!(!Solution::check_string("abba".to_string()));
    }

    #[test]
    fn test_only_a() {
        assert!(Solution::check_string("aaa".to_string()));
    }

    #[test]
    fn test_only_b() {
        assert!(Solution::check_string("bbb".to_string()));
    }

    #[test]
    fn test_single_char() {
        assert!(Solution::check_string("a".to_string()));
    }
}
