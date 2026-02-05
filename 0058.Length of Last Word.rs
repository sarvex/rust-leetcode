impl Solution {
    /// Reverse scan for length of the last word in a string.
    ///
    /// # Intuition
    /// Scanning from the end skips trailing spaces and immediately finds
    /// the last word boundary, avoiding processing the entire string.
    ///
    /// # Approach
    /// Trim trailing whitespace, then scan backwards from the end until
    /// a space is found or the string begins. The count of non-space
    /// characters is the last word's length.
    ///
    /// # Complexity
    /// - Time: O(n) — worst case scans the entire string
    /// - Space: O(1) — no additional allocation
    pub fn length_of_last_word(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .rev()
            .skip_while(|&&b| b == b' ')
            .take_while(|&&b| b != b' ')
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_spaces() {
        assert_eq!(
            Solution::length_of_last_word("Hello World   ".to_string()),
            5
        );
    }

    #[test]
    fn no_trailing_spaces() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn single_word() {
        assert_eq!(Solution::length_of_last_word("fly".to_string()), 3);
    }

    #[test]
    fn multiple_spaces_between() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
}
