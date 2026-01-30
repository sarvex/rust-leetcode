pub struct Solution;

impl Solution {
    /// Finds the longest dictionary word that is a subsequence of `s`.
    ///
    /// # Intuition
    /// Check each dictionary word as a subsequence of `s`. Among matches,
    /// prefer the longest, then lexicographically smallest.
    ///
    /// # Approach
    /// 1. For each word, use a two-pointer check against `s`.
    /// 2. Track the best match by length, then lexicographic order.
    ///
    /// # Complexity
    /// - Time: O(d × n) where d = dictionary size, n = s.len()
    /// - Space: O(1) extra
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s_bytes = s.as_bytes();

        fn is_subseq(word: &[u8], s: &[u8]) -> bool {
            let mut i = 0;
            for &b in s {
                if i < word.len() && word[i] == b {
                    i += 1;
                }
            }
            i == word.len()
        }

        let mut best = String::new();
        for word in &dictionary {
            if is_subseq(word.as_bytes(), s_bytes)
                && (word.len() > best.len() || (word.len() == best.len() && *word < best))
            {
                best = word.clone();
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec![
                    "ale".to_string(),
                    "apple".to_string(),
                    "monkey".to_string(),
                    "plea".to_string()
                ],
            ),
            "apple"
        );
    }

    #[test]
    fn test_tie() {
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
            ),
            "a"
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(
            Solution::find_longest_word("abc".to_string(), vec!["xyz".to_string()]),
            ""
        );
    }

    #[test]
    fn test_empty_dictionary() {
        assert_eq!(Solution::find_longest_word("abc".to_string(), vec![]), "");
    }

    #[test]
    fn test_lexicographic_order() {
        // Both "ba" and "ab" are valid, "ab" should win due to lexicographic order
        assert_eq!(
            Solution::find_longest_word(
                "abba".to_string(),
                vec!["ba".to_string(), "ab".to_string()],
            ),
            "ab"
        );
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(
            Solution::find_longest_word("word".to_string(), vec!["word".to_string()]),
            "word"
        );
    }
}
