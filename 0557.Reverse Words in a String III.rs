impl Solution {
    /// Reverses each word in a sentence while preserving word order.
    ///
    /// # Intuition
    /// Split by spaces, reverse each word's characters, and rejoin.
    ///
    /// # Approach
    /// 1. Split the string by spaces.
    /// 2. Map each word to its character-reversed form.
    /// 3. Join with spaces.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
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
}
