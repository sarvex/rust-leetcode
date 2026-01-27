use std::collections::HashSet;

impl Solution {
    /// Minimum string length after removing duplicate characters.
    ///
    /// # Intuition
    /// Each character can eliminate all other occurrences of itself, so the
    /// minimized length equals the number of distinct characters.
    ///
    /// # Approach
    /// 1. Collect all characters into a `HashSet`.
    /// 2. Return the set size.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(k) where k is the alphabet size
    pub fn minimized_string_length(s: String) -> i32 {
        s.bytes().collect::<HashSet<u8>>().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_unique_characters() {
        assert_eq!(Solution::minimized_string_length("abc".to_string()), 3);
    }

    #[test]
    fn repeated_characters_collapse() {
        assert_eq!(Solution::minimized_string_length("aaaa".to_string()), 1);
    }

    #[test]
    fn mixed_duplicates() {
        assert_eq!(Solution::minimized_string_length("ababc".to_string()), 3);
    }
}
