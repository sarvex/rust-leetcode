impl Solution {
    /// Checks if a sentence contains every letter of the alphabet.
    ///
    /// # Intuition
    /// Use a 26-bit mask to track which letters have been seen.
    /// A pangram has all 26 bits set.
    ///
    /// # Approach
    /// 1. Fold over bytes, setting the bit corresponding to each letter.
    /// 2. Check if the resulting mask equals (1 << 26) - 1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence
            .as_bytes()
            .iter()
            .fold(0u32, |mask, &b| mask | (1 << (b - b'a')))
            == (1 << 26) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pangram() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
    }

    #[test]
    fn test_not_pangram() {
        assert!(!Solution::check_if_pangram("hello".to_string()));
    }
}
