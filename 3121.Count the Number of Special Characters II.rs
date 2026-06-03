impl Solution {
    /// Count letters where all lowercase occurrences precede the first uppercase.
    ///
    /// # Intuition
    /// Two bitmasks suffice: track letters that have appeared uppercase (`upper`),
    /// and letters that are already disqualified (`bad`). A letter is disqualified
    /// the instant we see its lowercase form after its uppercase has appeared.
    /// Letters never seen in lowercase are excluded by the final `& !bad` only
    /// counting letters in `upper` that also had a valid lowercase — captured by
    /// a third `lower` mask, but we can fold `lower` away: a letter contributes
    /// to the answer iff it is in `upper`, was seen lowercase at least once, and
    /// was never disqualified. Tracking `lower` explicitly is the clearest path.
    ///
    /// # Approach
    /// Single forward pass with three `u32` bitmasks (12 bytes, register-resident):
    /// - `lower` — set when a lowercase byte is seen.
    /// - `upper` — set when an uppercase byte is seen.
    /// - `bad`   — set when a lowercase byte is seen whose `upper` bit is already
    ///             set (i.e., its uppercase appeared earlier in the string).
    ///
    /// Answer: `(lower & upper & !bad).count_ones()`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn number_of_special_chars(word: String) -> i32 {
        let (mut lower, mut upper, mut bad) = (0u32, 0u32, 0u32);
        for b in word.bytes() {
            if b >= b'a' {
                let bit = 1u32 << (b - b'a');
                bad |= bit & upper; // lowercase after its uppercase → disqualify
                lower |= bit;
            } else {
                upper |= 1u32 << (b - b'A');
            }
        }
        (lower & upper & !bad).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 'a', 'b', 'c' are all special
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        // no uppercase at all
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
    }

    #[test]
    fn test_example_3() {
        // 'a' has lowercase after uppercase → not special; 'b'/'c' same issue
        assert_eq!(Solution::number_of_special_chars("AbBCab".to_string()), 0);
    }

    #[test]
    fn test_all_uppercase() {
        assert_eq!(Solution::number_of_special_chars("ABC".to_string()), 0);
    }

    #[test]
    fn test_all_lowercase() {
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
    }

    #[test]
    fn test_single_pair_valid() {
        assert_eq!(Solution::number_of_special_chars("aA".to_string()), 1);
    }

    #[test]
    fn test_single_pair_invalid() {
        // uppercase before lowercase → not special
        assert_eq!(Solution::number_of_special_chars("Aa".to_string()), 0);
    }

    #[test]
    fn test_all_26_special() {
        assert_eq!(
            Solution::number_of_special_chars(
                "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ".to_string()
            ),
            26
        );
    }

    #[test]
    fn test_interleaved_invalid() {
        // 'a' appears lowercase after its uppercase → not special
        assert_eq!(Solution::number_of_special_chars("aAa".to_string()), 0);
    }
}
