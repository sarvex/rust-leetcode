impl Solution {
    /// Trims trailing vowels from the string using `trim_end_matches`.
    ///
    /// # Intuition
    /// Trailing vowels are simply a suffix of consecutive vowel characters.
    /// Rust's `trim_end_matches` strips characters from the end while the
    /// predicate holds, which is exactly the operation needed.
    ///
    /// # Approach
    /// 1. Use `trim_end_matches` with a closure that checks membership in
    ///    `{'a', 'e', 'i', 'o', 'u'}`.
    /// 2. Convert the resulting `&str` back to an owned `String`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the returned string
    pub fn trim_trailing_vowels(s: String) -> String {
        s.trim_end_matches(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_vowels() {
        assert_eq!(Solution::trim_trailing_vowels("idea".to_string()), "id");
    }

    #[test]
    fn test_no_trailing_vowels() {
        assert_eq!(Solution::trim_trailing_vowels("day".to_string()), "day");
    }

    #[test]
    fn test_all_vowels() {
        assert_eq!(Solution::trim_trailing_vowels("aeiou".to_string()), "");
    }

    #[test]
    fn test_single_consonant() {
        assert_eq!(Solution::trim_trailing_vowels("b".to_string()), "b");
    }

    #[test]
    fn test_single_vowel() {
        assert_eq!(Solution::trim_trailing_vowels("a".to_string()), "");
    }

    #[test]
    fn test_mixed_interior_vowels() {
        assert_eq!(
            Solution::trim_trailing_vowels("beautiful".to_string()),
            "beautif"
        );
    }
}
