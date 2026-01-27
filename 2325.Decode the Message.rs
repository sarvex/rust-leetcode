impl Solution {
    /// Decodes a message using a substitution cipher from a key string.
    ///
    /// # Intuition
    /// The key defines a mapping from its first-occurrence letters to the alphabet.
    /// Spaces are preserved as-is.
    ///
    /// # Approach
    /// 1. Build a substitution table from the key's first occurrences
    /// 2. Map each message character through the table
    ///
    /// # Complexity
    /// - Time: O(n + m) where n = key length, m = message length
    /// - Space: O(1) — fixed 26-entry lookup table
    pub fn decode_message(key: String, message: String) -> String {
        let mut table = [b' '; 256];
        let mut next_char = b'a';

        for &c in key.as_bytes() {
            if c != b' ' && table[c as usize] == b' ' {
                table[c as usize] = next_char;
                next_char += 1;
            }
        }
        table[b' ' as usize] = b' ';

        message
            .as_bytes()
            .iter()
            .map(|&c| table[c as usize] as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_decode() {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepdings".to_string();
        assert_eq!(
            Solution::decode_message(key, message),
            "this is a]secret".to_string().replace(']', " ")
        );
    }

    #[test]
    fn test_spaces_preserved() {
        let key = "abcdefghijklmnopqrstuvwxyz".to_string();
        let message = "a b c".to_string();
        assert_eq!(Solution::decode_message(key, message), "a b c");
    }

    #[test]
    fn test_identity_mapping() {
        let key = "abcdefghijklmnopqrstuvwxyz".to_string();
        let message = "hello".to_string();
        assert_eq!(Solution::decode_message(key, message), "hello");
    }
}
