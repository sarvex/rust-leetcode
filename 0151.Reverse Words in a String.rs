impl Solution {
    /// Reverses the order of words in a string using a single-pass fold.
    ///
    /// # Intuition
    /// Split on whitespace to extract words in reverse, then fold them
    /// into a preallocated string without intermediate collection.
    ///
    /// # Approach
    /// 1. Split the string on whitespace (handles multiple spaces).
    /// 2. Reverse the iterator and fold directly into a preallocated `String`.
    /// 3. Append each word with a space separator.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the output string
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .rev()
            .fold(String::with_capacity(s.len()), |mut reversed, word| {
                if !reversed.is_empty() {
                    reversed.push(' ');
                }
                reversed.push_str(word);
                reversed
            })
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
