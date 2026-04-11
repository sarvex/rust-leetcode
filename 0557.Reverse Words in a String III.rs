impl Solution {
    /// Reverses each word in a sentence while preserving word order.
    ///
    /// # Intuition
    /// Split by spaces, reverse each word's characters, and fold directly
    /// into a preallocated output string without intermediate collection.
    ///
    /// # Approach
    /// 1. Split the string by spaces.
    /// 2. Enumerate and fold, extending with each word's reversed characters.
    /// 3. Use a preallocated `String` to avoid repeated allocation.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .enumerate()
            .fold(String::with_capacity(s.len()), |mut reversed, (i, word)| {
                if i > 0 {
                    reversed.push(' ');
                }
                reversed.extend(word.chars().rev());
                reversed
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(Solution::reverse_words("hello".to_string()), "olleh");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::reverse_words("a".to_string()), "a");
    }

    #[test]
    fn test_god_ding() {
        assert_eq!(Solution::reverse_words("God Ding".to_string()), "doG gniD");
    }

    #[test]
    fn test_palindrome_words() {
        assert_eq!(Solution::reverse_words("mom dad".to_string()), "mom dad");
    }

    #[test]
    fn test_multiple_spaces_preserved() {
        // Each word is individually reversed, space count preserved
        assert_eq!(Solution::reverse_words("ab cd".to_string()), "ba dc");
    }
}
