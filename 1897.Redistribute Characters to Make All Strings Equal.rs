impl Solution {
    /// Checks if characters can be redistributed to make all strings equal.
    ///
    /// # Intuition
    /// If each character's total count is divisible by the number of words,
    /// we can evenly distribute characters across all words.
    ///
    /// # Approach
    /// 1. Count total frequency of each character across all words.
    /// 2. Check that every frequency is divisible by the word count.
    ///
    /// # Complexity
    /// - Time: O(n) where n is total characters
    /// - Space: O(1) — 26-entry frequency array
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = words.len();
        let mut freq = [0usize; 26];

        for word in &words {
            for &b in word.as_bytes() {
                freq[(b - b'a') as usize] += 1;
            }
        }

        freq.iter().all(|&count| count % n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible() {
        assert!(Solution::make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::make_equal(vec![
            "ab".to_string(),
            "a".to_string()
        ]));
    }

    #[test]
    fn test_single_word() {
        assert!(Solution::make_equal(vec!["hello".to_string()]));
    }
}
