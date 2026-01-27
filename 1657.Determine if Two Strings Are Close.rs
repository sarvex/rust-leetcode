impl Solution {
    /// Frequency multiset and character set comparison.
    ///
    /// # Intuition
    /// Two strings are close if they share the same character set and have
    /// the same multiset of frequencies (since swaps can reorder characters
    /// and transforms can redistribute frequencies among existing characters).
    ///
    /// # Approach
    /// 1. Count character frequencies for both strings
    /// 2. Verify same characters are present (non-zero positions match)
    /// 3. Sort frequency arrays and compare
    ///
    /// # Complexity
    /// - Time: O(n + 26 log 26) ≈ O(n)
    /// - Space: O(1) — fixed-size arrays
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut freq1 = [0i32; 26];
        let mut freq2 = [0i32; 26];

        for b in word1.bytes() {
            freq1[(b - b'a') as usize] += 1;
        }
        for b in word2.bytes() {
            freq2[(b - b'a') as usize] += 1;
        }

        if (0..26).any(|i| (freq1[i] == 0) != (freq2[i] == 0)) {
            return false;
        }

        freq1.sort_unstable();
        freq2.sort_unstable();
        freq1 == freq2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_strings() {
        assert!(Solution::close_strings(
            "abc".to_string(),
            "bca".to_string()
        ));
    }

    #[test]
    fn not_close_different_chars() {
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn close_with_transform() {
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string(),
        ));
    }
}
