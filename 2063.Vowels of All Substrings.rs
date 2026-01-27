impl Solution {
    /// Counts total vowel occurrences across all substrings.
    ///
    /// # Intuition
    /// Each vowel at index i contributes to (i+1) * (n-i) substrings.
    ///
    /// # Approach
    /// 1. For each vowel character, compute its contribution.
    /// 2. Sum all contributions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_vowels(word: String) -> i64 {
        let n = word.len() as i64;
        word.bytes()
            .enumerate()
            .filter(|(_, b)| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u'))
            .map(|(i, _)| (i as i64 + 1) * (n - i as i64))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::count_vowels("aba".to_string()), 6);
    }

    #[test]
    fn test_single_vowel() {
        assert_eq!(Solution::count_vowels("a".to_string()), 1);
    }

    #[test]
    fn test_no_vowels() {
        assert_eq!(Solution::count_vowels("bcd".to_string()), 0);
    }
}
