impl Solution {
    /// Minimum string length after removing duplicate characters.
    ///
    /// # Intuition
    /// Each character can eliminate all other occurrences of itself, so the
    /// minimized length equals the number of distinct characters.
    ///
    /// # Approach
    /// 1. Mark all characters in a fixed ASCII array.
    /// 2. Count the marked entries.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed-size ASCII lookup array
    pub fn minimized_string_length(s: String) -> i32 {
        let mut seen = [false; 128];
        s.bytes().for_each(|b| seen[b as usize] = true);
        seen.iter().filter(|&&is_seen| is_seen).count() as i32
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
