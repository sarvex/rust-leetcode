impl Solution {
    /// Reverses the prefix of a word up to the first occurrence of a character.
    ///
    /// # Intuition
    /// Find the target character and reverse the substring from start to
    /// that position inclusive.
    ///
    /// # Approach
    /// 1. Find the first occurrence of ch.
    /// 2. Reverse the prefix up to and including that index.
    /// 3. Concatenate with the remaining suffix.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(i) => word[..=i].chars().rev().collect::<String>() + &word[i + 1..],
            None => word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd"
        );
    }

    #[test]
    fn test_not_found() {
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd");
    }

    #[test]
    fn test_first_char() {
        assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'a'), "abcd");
    }
}
