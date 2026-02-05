impl Solution {
    /// Finds the maximum number of non-overlapping palindrome substrings of length >= k.
    ///
    /// # Intuition
    /// Process the string from right to left, expanding around each center to find
    /// palindromes. By going backwards, we can immediately use previously computed
    /// results for positions to the right.
    ///
    /// # Approach
    /// 1. Use right-to-left DP where `counts[i]` = max palindromes from index i to end
    /// 2. For each position, try expanding as center for odd-length palindromes
    /// 3. Also try even-length palindromes centered between adjacent equal characters
    /// 4. Update counts when a valid palindrome of length >= k is found
    /// 5. Propagate maximum counts leftward
    ///
    /// # Complexity
    /// - Time: O(n^2) worst case with early termination on mismatch
    /// - Space: O(n) for the counts array
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut counts = vec![0; n + 1];

        (0..n).rev().for_each(|i| {
            Self::expand_from(bytes, k, &mut counts, i, i);

            if i + 1 < n && bytes[i + 1] == bytes[i] {
                Self::expand_from(bytes, k, &mut counts, i, i + 1);
            }

            counts[i] = counts[i].max(counts[i + 1]);
        });

        counts[0]
    }

    fn expand_from(bytes: &[u8], k: i32, counts: &mut [i32], left: usize, right: usize) {
        let mut extra = 0;

        while left >= extra
            && right + extra < bytes.len()
            && bytes[left - extra] == bytes[right + extra]
        {
            let palindrome_len = right - left + 1 + extra * 2;

            if palindrome_len >= k as usize {
                let start_idx = left - extra;
                let end_idx = right + extra + 1;
                counts[start_idx] = counts[start_idx].max(counts[end_idx] + 1);
            }

            extra += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_palindromes("abaccdbbd".to_string(), 3), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_palindromes("adbcda".to_string(), 2), 0);
    }

    #[test]
    fn test_single_palindrome() {
        assert_eq!(Solution::max_palindromes("aba".to_string(), 3), 1);
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(Solution::max_palindromes("aaa".to_string(), 2), 1);
    }

    #[test]
    fn test_four_same_chars() {
        assert_eq!(Solution::max_palindromes("aaaa".to_string(), 2), 2);
    }

    #[test]
    fn test_k_exceeds_length() {
        assert_eq!(Solution::max_palindromes("ab".to_string(), 3), 0);
    }

    #[test]
    fn test_multiple_short_palindromes() {
        assert_eq!(Solution::max_palindromes("aabbcc".to_string(), 2), 3);
    }

    #[test]
    fn test_single_char_k_one() {
        assert_eq!(Solution::max_palindromes("a".to_string(), 1), 1);
    }

    #[test]
    fn test_nested_palindromes() {
        assert_eq!(Solution::max_palindromes("abacaba".to_string(), 3), 2);
    }

    #[test]
    fn test_racecar() {
        assert_eq!(Solution::max_palindromes("racecar".to_string(), 3), 1);
    }
}
