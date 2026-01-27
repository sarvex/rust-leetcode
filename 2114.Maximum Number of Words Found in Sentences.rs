impl Solution {
    /// Finds the maximum word count across sentences by counting spaces.
    ///
    /// # Intuition
    /// Words in a sentence are separated by single spaces, so the word count
    /// equals the number of spaces plus one.
    ///
    /// # Approach
    /// 1. For each sentence, count space bytes and add one.
    /// 2. Return the maximum across all sentences.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is sentence count, m is average sentence length
    /// - Space: O(1)
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.as_bytes().iter().filter(|&&b| b == b' ').count() as i32 + 1)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let sentences = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(Solution::most_words_found(sentences), 6);
    }

    #[test]
    fn test_single_word_sentences() {
        let sentences = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(Solution::most_words_found(sentences), 1);
    }

    #[test]
    fn test_equal_length_sentences() {
        let sentences = vec!["a b".to_string(), "c d".to_string()];
        assert_eq!(Solution::most_words_found(sentences), 2);
    }
}
