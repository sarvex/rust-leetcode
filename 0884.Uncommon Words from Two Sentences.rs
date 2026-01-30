use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Finds words that appear exactly once across both sentences.
    ///
    /// # Intuition
    /// A word is uncommon if it appears exactly once in the combined corpus
    /// of both sentences.
    ///
    /// # Approach
    /// Count word frequencies across both sentences. Collect words with
    /// frequency exactly 1.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n and m are sentence lengths
    /// - Space: O(n + m) for the frequency map
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        // Estimate capacity based on typical word count
        let estimated_words = (s1.len() + s2.len()) / 5; // Average word length ~5 chars
        let mut freq: HashMap<&str, usize> = HashMap::with_capacity(estimated_words);
        for word in s1.split_whitespace().chain(s2.split_whitespace()) {
            *freq.entry(word).or_default() += 1;
        }
        freq.into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(word, _)| word.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut result = Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        result.sort();
        assert_eq!(result, vec!["sour", "sweet"]);
    }

    #[test]
    fn test_duplicates() {
        let result =
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string());
        assert_eq!(result, vec!["banana"]);
    }
}
