impl Solution {
    /// Checks if a string is an acronym of the given words.
    ///
    /// # Intuition
    /// An acronym is formed by concatenating the first character of each word.
    /// Collecting these characters and comparing to `s` gives the answer.
    ///
    /// # Approach
    /// 1. Check length equality first (early exit).
    /// 2. Zip words with string bytes, comparing each first character.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of words
    /// - Space: O(1)
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words.len() == s.len()
            && words
                .iter()
                .zip(s.as_bytes())
                .all(|(w, b)| w.as_bytes()[0] == *b)
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
