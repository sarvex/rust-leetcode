impl Solution {
    /// Map words to characters based on weighted character sums.
    ///
    /// # Intuition
    /// Each word is transformed into a single character by:
    /// 1. Computing the sum of character weights
    /// 2. Taking modulo 26 to get a value in [0, 25]
    /// 3. Mapping to reverse alphabetical order (0→'z', 1→'y', ..., 25→'a')
    ///
    /// # Approach
    /// - For each word, iterate through its characters
    /// - Sum up the weights using the weights array (indexed by char - 'a')
    /// - Take the sum modulo 26
    /// - Convert to character using reverse mapping: 'z' - (sum % 26) as u8
    /// - Collect all mapped characters into a result string
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is the number of words and m is average word length
    /// - Space: O(n) for the result string
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .iter()
            .map(|word| {
                let weight_sum: i32 = word
                    .chars()
                    .map(|c| weights[(c as u8 - b'a') as usize])
                    .sum();
                let index = (weight_sum % 26) as u8;
                (b'z' - index) as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()];
        let weights = vec![
            5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
        ];
        assert_eq!(Solution::map_word_weights(words, weights), "rij");
    }

    #[test]
    fn test_example_2() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let weights = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        assert_eq!(Solution::map_word_weights(words, weights), "yyy");
    }

    #[test]
    fn test_example_3() {
        let words = vec!["abcd".to_string()];
        let weights = vec![
            7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
        ];
        assert_eq!(Solution::map_word_weights(words, weights), "g");
    }

    #[test]
    fn test_single_character_words() {
        let words = vec!["z".to_string()];
        let weights = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 26,
        ];
        // 'z' has weight 26, 26 % 26 = 0, maps to 'z'
        assert_eq!(Solution::map_word_weights(words, weights), "z");
    }

    #[test]
    fn test_edge_modulo_25() {
        let words = vec!["a".to_string()];
        let weights = vec![
            25, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        // 'a' has weight 25, 25 % 26 = 25, maps to 'a'
        assert_eq!(Solution::map_word_weights(words, weights), "a");
    }

    #[test]
    fn test_multiple_words_max_length() {
        let words = vec!["abcdefghij".to_string(), "klmnopqrst".to_string()];
        let weights = vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ];
        // Each word: 10 chars * weight 10 = 100, 100 % 26 = 22, maps to 'd'
        assert_eq!(Solution::map_word_weights(words, weights), "dd");
    }

    #[test]
    fn test_large_weight_sum() {
        let words = vec!["aaaa".to_string()];
        let weights = vec![
            100, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        // 'aaaa' has weight 400, 400 % 26 = 10, maps to 'p'
        assert_eq!(Solution::map_word_weights(words, weights), "p");
    }
}
