impl Solution {
    /// Counts substrings that begin and end with the same letter.
    ///
    /// # Intuition
    /// Each occurrence of a character can form a valid substring with every
    /// previous occurrence of the same character (plus itself).
    ///
    /// # Approach
    /// 1. Maintain a count of each character seen so far.
    /// 2. For each character, increment its count and add that count.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — 26-entry array
    pub fn number_of_substrings(s: String) -> i64 {
        let mut freq = [0i64; 26];
        let mut result = 0i64;

        for &b in s.as_bytes() {
            let idx = (b - b'a') as usize;
            freq[idx] += 1;
            result += freq[idx];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::number_of_substrings("abcba".to_string()), 7);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::number_of_substrings("a".to_string()), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::number_of_substrings("aaa".to_string()), 6);
    }
}
