impl Solution {
    /// Finds minimum flips to make a binary string alternating using a single-counter sliding window.
    ///
    /// # Intuition
    /// Track mismatches against one pattern ("010..."); the other is always `len - mismatches`.
    /// When length is even, rotation preserves mismatch parity so the answer is immediate.
    /// When length is odd, each rotation shifts one character whose entering and leaving
    /// parities differ, producing a ±1 delta computable via a single XOR-and-mask.
    ///
    /// # Approach
    /// 1. Count mismatches against "010..." using `(byte ^ index) & 1`.
    /// 2. If length is even, return `min(mismatches, len - mismatches)` directly.
    /// 3. For odd length, slide through all rotations: the entering and leaving character
    ///    are identical, only their parity flips, giving `delta = 1 - 2 * ((byte ^ k) & 1)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_flips(s: String) -> i32 {
        let length = s.len();
        let bytes = s.as_bytes();

        let mut mismatches: i32 = bytes
            .iter()
            .enumerate()
            .map(|(i, byte)| ((*byte as usize ^ i) & 1) as i32)
            .sum();

        let length_i32 = length as i32;
        let mut result = mismatches.min(length_i32 - mismatches);

        if length & 1 == 0 {
            return result;
        }

        for (k, byte) in bytes.iter().enumerate() {
            mismatches += 1 - 2 * (((*byte as usize ^ k) & 1) as i32);
            result = result.min(mismatches.min(length_i32 - mismatches));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_needed() {
        assert_eq!(Solution::min_flips("111000".to_string()), 2);
    }

    #[test]
    fn test_already_alternating() {
        assert_eq!(Solution::min_flips("010".to_string()), 0);
    }

    #[test]
    fn test_single_flip() {
        assert_eq!(Solution::min_flips("1110".to_string()), 1);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::min_flips("0".to_string()), 0);
        assert_eq!(Solution::min_flips("1".to_string()), 0);
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(Solution::min_flips("01".to_string()), 0);
        assert_eq!(Solution::min_flips("10".to_string()), 0);
        assert_eq!(Solution::min_flips("00".to_string()), 1);
        assert_eq!(Solution::min_flips("11".to_string()), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::min_flips("00000".to_string()), 2);
        assert_eq!(Solution::min_flips("1111".to_string()), 2);
    }
}
