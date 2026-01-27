impl Solution {
    /// Counts words that can be fully typed without broken keys.
    ///
    /// # Intuition
    /// Mark broken letters in a lookup array. For each word, check if
    /// any letter is broken.
    ///
    /// # Approach
    /// 1. Build a boolean array of broken letters.
    /// 2. Split text by whitespace and count words with no broken letters.
    ///
    /// # Complexity
    /// - Time: O(n) where n is total text length
    /// - Space: O(1) — 26-entry boolean array
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = [false; 26];
        for &b in broken_letters.as_bytes() {
            broken[(b - b'a') as usize] = true;
        }

        text.split_whitespace()
            .filter(|word| word.bytes().all(|b| !broken[(b - b'a') as usize]))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_broken() {
        assert_eq!(
            Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()),
            1
        );
    }

    #[test]
    fn test_none_broken() {
        assert_eq!(
            Solution::can_be_typed_words("hello world".to_string(), "".to_string()),
            2
        );
    }

    #[test]
    fn test_all_broken() {
        assert_eq!(
            Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()),
            1
        );
    }
}
