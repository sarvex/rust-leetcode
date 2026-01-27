impl Solution {
    /// Converts a string to lowercase using ASCII conversion.
    ///
    /// # Intuition
    /// Rust's standard library provides efficient ASCII lowercase conversion.
    ///
    /// # Approach
    /// Delegate to `to_ascii_lowercase` which handles the byte-level conversion.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the new string
    pub fn to_lower_case(s: String) -> String {
        s.to_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_case() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
    }

    #[test]
    fn test_already_lowercase() {
        assert_eq!(Solution::to_lower_case("here".to_string()), "here");
    }

    #[test]
    fn test_with_non_alpha() {
        assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::to_lower_case(String::new()), "");
    }
}
