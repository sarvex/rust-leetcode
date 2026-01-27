use std::collections::HashMap;

impl Solution {
    /// Checks if a pattern matches a string of words using bidirectional mapping.
    ///
    /// # Intuition
    /// Each pattern character must map to exactly one word and vice versa.
    /// Track first-occurrence indices for both to verify bijection.
    ///
    /// # Approach
    /// 1. Split the string into words.
    /// 2. If counts differ, return false.
    /// 3. Map each character and word to their first occurrence index.
    /// 4. If mapped indices differ at any position, the pattern doesn't match.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the pattern/word count
    /// - Space: O(n) for the maps
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let chars: Vec<char> = pattern.chars().collect();
        let words: Vec<&str> = s.split_whitespace().collect();
        if chars.len() != words.len() {
            return false;
        }
        let mut char_map = HashMap::new();
        let mut word_map = HashMap::new();
        for (i, (&c, &w)) in chars.iter().zip(words.iter()).enumerate() {
            let char_idx = *char_map.entry(c).or_insert(i);
            let word_idx = *word_map.entry(w).or_insert(i);
            if char_idx != word_idx {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_pattern() {
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn non_matching() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
    }

    #[test]
    fn different_lengths() {
        assert!(!Solution::word_pattern(
            "abc".to_string(),
            "dog cat".to_string()
        ));
    }
}
