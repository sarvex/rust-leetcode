impl Solution {
    /// Checks if a string is an acronym of the given words.
    ///
    /// # Intuition
    /// An acronym is formed by concatenating the first character of each word.
    /// Collecting these characters and comparing to `s` gives the answer.
    ///
    /// # Approach
    /// 1. Map each word to its first character.
    /// 2. Collect into a `String` and compare with `s`.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of words
    /// - Space: O(n) for the collected acronym string
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words
            .iter()
            .map(|w| w.as_bytes()[0] as char)
            .collect::<String>()
            == s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_acronym() {
        let words = vec![
            "alice".to_string(),
            "bob".to_string(),
            "charlie".to_string(),
        ];
        assert!(Solution::is_acronym(words, "abc".to_string()));
    }

    #[test]
    fn test_invalid_acronym() {
        let words = vec!["an".to_string(), "apple".to_string()];
        assert!(!Solution::is_acronym(words, "a".to_string()));
    }

    #[test]
    fn test_single_word() {
        let words = vec!["never".to_string()];
        assert!(Solution::is_acronym(words, "n".to_string()));
    }
}
