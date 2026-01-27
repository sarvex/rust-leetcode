impl Solution {
    /// Reverses the order of words in a string using split and collect.
    ///
    /// # Intuition
    /// Split on whitespace to extract words, then reverse their order
    /// and rejoin with single spaces.
    ///
    /// # Approach
    /// 1. Split the string on whitespace (handles multiple spaces).
    /// 2. Collect non-empty words into a vector.
    /// 3. Reverse the vector and join with a single space.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the words collection
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    #[test]
    fn leading_trailing_spaces() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello"
        );
    }

    #[test]
    fn multiple_spaces_between() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
