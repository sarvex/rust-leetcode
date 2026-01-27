use std::collections::{HashMap, HashSet};

impl Solution {
    /// Finds the most frequent non-banned word in a paragraph.
    ///
    /// # Intuition
    /// Normalize to lowercase, split on non-alphabetic characters, filter
    /// banned words, and count frequencies.
    ///
    /// # Approach
    /// Convert paragraph to lowercase. Split by non-alpha delimiters. Use a
    /// `HashSet` for banned words and a `HashMap` for frequency counting.
    /// Return the word with the highest count.
    ///
    /// # Complexity
    /// - Time: O(n + b) where n is paragraph length and b is banned list size
    /// - Space: O(n + b)
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let paragraph = paragraph.to_ascii_lowercase();
        let banned: HashSet<&str> = banned.iter().map(String::as_str).collect();
        let mut freq: HashMap<&str, usize> = HashMap::new();

        for word in paragraph.split(|c: char| !c.is_ascii_lowercase()) {
            if !word.is_empty() && !banned.contains(word) {
                *freq.entry(word).or_default() += 1;
            }
        }

        freq.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(word, _)| word.to_string())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()],
            ),
            "ball"
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(
            Solution::most_common_word("a.".to_string(), Vec::new()),
            "a"
        );
    }
}
