impl Solution {
    /// Checks if a sentence is circular (last char of each word matches first of next).
    ///
    /// # Intuition
    /// In a circular sentence, every word's last character must equal the next word's
    /// first character, wrapping around from the last word to the first.
    ///
    /// # Approach
    /// Split into words, use windows of adjacent pairs plus wrap-around check.
    /// Alternatively, check that every space is preceded and followed by the same character.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — only inspects bytes at space boundaries
    pub fn is_circular_sentence(sentence: String) -> bool {
        let bytes = sentence.as_bytes();
        let n = bytes.len();

        if bytes[0] != bytes[n - 1] {
            return false;
        }

        bytes
            .windows(3)
            .filter(|w| w[1] == b' ')
            .all(|w| w[0] == w[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises strong coding".to_string()
        ));
    }

    #[test]
    fn test_not_circular() {
        assert!(!Solution::is_circular_sentence(
            "Leetcode is cool".to_string()
        ));
    }

    #[test]
    fn test_single_word() {
        assert!(Solution::is_circular_sentence("aba".to_string()));
    }

    #[test]
    fn test_single_word_not_circular() {
        assert!(!Solution::is_circular_sentence("ab".to_string()));
    }
}
