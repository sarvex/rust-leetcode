impl Solution {
    /// Checks whether both halves of a string contain equal vowel counts.
    ///
    /// # Intuition
    /// Split the string in half and compare vowel frequencies. A single counter
    /// incremented for the first half and decremented for the second suffices.
    ///
    /// # Approach
    /// 1. Convert to bytes for efficient iteration.
    /// 2. For each position, check if the byte is a vowel.
    /// 3. Increment the balance for the first half, decrement for the second.
    /// 4. Return whether the balance is zero.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn halves_are_alike(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let is_vowel = |b: u8| {
            matches!(
                b,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };
        let balance: i32 = bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| {
                if !is_vowel(b) {
                    return 0;
                }
                if i < n / 2 {
                    1
                } else {
                    -1
                }
            })
            .sum();
        balance == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_book() {
        assert!(Solution::halves_are_alike("book".to_string()));
    }

    #[test]
    fn test_example_textbook() {
        assert!(!Solution::halves_are_alike("textbook".to_string()));
    }

    #[test]
    fn test_no_vowels() {
        assert!(Solution::halves_are_alike("bcdf".to_string()));
    }
}
