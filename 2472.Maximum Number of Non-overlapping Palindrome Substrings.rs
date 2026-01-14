impl Solution {
    /// Maximum Number of Non-overlapping Palindrome Substrings
    ///
    /// # Intuition
    /// Process the string from right to left, expanding around each center to find
    /// palindromes. By going backwards, we can immediately use previously computed
    /// results for positions to the right.
    ///
    /// # Approach
    /// 1. Use right-to-left DP where `counts[i]` = max palindromes from index i to end
    /// 2. For each position, try expanding around it as center (odd-length palindromes)
    /// 3. Also try expanding around adjacent equal characters (even-length palindromes)
    /// 4. Update counts when a valid palindrome of length >= k is found
    /// 5. Propagate maximum counts leftward
    ///
    /// # Complexity
    /// - Time: O(n²) worst case, but with early termination on mismatch
    /// - Space: O(n) for the counts array
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut counts = vec![0; n + 1];

        for i in (0..n).rev() {
            // Try odd-length palindromes centered at i
            Self::expand_from(bytes, k, &mut counts, i, i);

            // Try even-length palindromes centered between i and i+1
            if i + 1 < n && bytes[i + 1] == bytes[i] {
                Self::expand_from(bytes, k, &mut counts, i, i + 1);
            }

            // Propagate maximum from right
            counts[i] = counts[i].max(counts[i + 1]);
        }

        counts[0]
    }

    fn expand_from(bytes: &[u8], k: i32, counts: &mut [i32], left: usize, right: usize) {
        let mut extra_chars = 0;

        while left >= extra_chars
            && right + extra_chars < bytes.len()
            && bytes[left - extra_chars] == bytes[right + extra_chars]
        {
            let palindrome_len = right - left + 1 + extra_chars * 2;

            if palindrome_len >= k as usize {
                let start_idx = left - extra_chars;
                let end_idx = right + extra_chars + 1;
                counts[start_idx] = counts[start_idx].max(counts[end_idx] + 1);
            }

            extra_chars += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "aba" and "dbbd" are valid palindromes
        let result = Solution::max_palindromes("abaccdbbd".to_string(), 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        // No palindrome substring of length >= 2
        let result = Solution::max_palindromes("adbcda".to_string(), 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_palindrome() {
        let result = Solution::max_palindromes("aba".to_string(), 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_same_characters() {
        let result = Solution::max_palindromes("aaa".to_string(), 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_longer_same_characters() {
        // "aaaa" can have "aa" and "aa" as two non-overlapping palindromes
        let result = Solution::max_palindromes("aaaa".to_string(), 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_k_equals_length() {
        let result = Solution::max_palindromes("aba".to_string(), 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_k_greater_than_length() {
        let result = Solution::max_palindromes("ab".to_string(), 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_multiple_short_palindromes() {
        // "aa", "bb", "cc" - three non-overlapping palindromes
        let result = Solution::max_palindromes("aabbcc".to_string(), 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_alternating_pattern() {
        let result = Solution::max_palindromes("abab".to_string(), 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_nested_palindromes() {
        let result = Solution::max_palindromes("abacaba".to_string(), 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_single_char_with_k_one() {
        let result = Solution::max_palindromes("a".to_string(), 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_long_palindrome() {
        let result = Solution::max_palindromes("racecar".to_string(), 3);
        assert_eq!(result, 1);
    }
}
