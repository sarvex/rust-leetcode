impl Solution {
    /// Checks if the string is formed by repeating a substring.
    ///
    /// # Intuition
    /// If `s` is built from a repeated pattern, then `s` appears in `(s + s)`
    /// at a position other than 0 and len. Removing the first and last
    /// character of the doubled string and checking for `s` suffices.
    ///
    /// # Approach
    /// 1. Concatenate `s` with itself.
    /// 2. Trim the first and last characters.
    /// 3. Check if `s` exists within the trimmed string.
    ///
    /// # Complexity
    /// - Time: O(n) with KMP-based contains, O(n²) worst-case naive
    /// - Space: O(n)
    pub fn repeated_substring_pattern(s: String) -> bool {
        let doubled = format!("{s}{s}");
        doubled[1..doubled.len() - 1].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test_not_repeated() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test_triple() {
        assert!(Solution::repeated_substring_pattern(
            "abcabcabc".to_string()
        ));
    }
}
