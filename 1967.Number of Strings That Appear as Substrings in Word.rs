impl Solution {
    /// Counts how many patterns appear as substrings of the word.
    ///
    /// # Intuition
    /// Simple substring containment check for each pattern.
    ///
    /// # Approach
    /// 1. Filter patterns that are contained in the word.
    /// 2. Return the count.
    ///
    /// # Complexity
    /// - Time: O(p * n) where p is total pattern length, n is word length
    /// - Space: O(1)
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|p| word.contains(p.as_str()))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
    }

    #[test]
    fn test_none_match() {
        assert_eq!(
            Solution::num_of_strings(vec!["x".to_string(), "y".to_string()], "abc".to_string()),
            0
        );
    }
}
