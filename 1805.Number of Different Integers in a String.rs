use std::collections::HashSet;

impl Solution {
    /// Counts distinct integers embedded in a string, ignoring leading zeros.
    ///
    /// # Intuition
    /// Extract digit sequences, strip leading zeros, and use a set for
    /// deduplication. String comparison handles arbitrarily large numbers.
    ///
    /// # Approach
    /// 1. Scan bytes to find contiguous digit runs.
    /// 2. Strip leading zeros from each run.
    /// 3. Insert the normalized string into a hash set.
    /// 4. Return the set size.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn num_different_integers(word: String) -> i32 {
        let bytes = word.as_bytes();
        let n = bytes.len();
        let mut set = HashSet::new();
        let mut i = 0;

        while i < n {
            if bytes[i].is_ascii_digit() {
                let start = i;
                while i < n && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let mut leading = start;
                while leading < i - 1 && bytes[leading] == b'0' {
                    leading += 1;
                }
                set.insert(&word[leading..i]);
            } else {
                i += 1;
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_string() {
        assert_eq!(
            Solution::num_different_integers("a123bc34d8ef34".to_string()),
            3
        );
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
    }

    #[test]
    fn test_no_digits() {
        assert_eq!(Solution::num_different_integers("abc".to_string()), 0);
    }
}
