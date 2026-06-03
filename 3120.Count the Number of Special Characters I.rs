impl Solution {
    /// Count letters appearing in both lowercase and uppercase using bitmask.
    ///
    /// # Intuition
    /// Track which letters appear as lowercase and which as uppercase using two
    /// bitmasks, then count bits set in both.
    ///
    /// # Approach
    /// Iterate through the word once, setting bits in `lower` for lowercase letters
    /// and `upper` for uppercase letters. The answer is the popcount of their AND.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn number_of_special_chars(word: String) -> i32 {
        let (mut lower, mut upper) = (0u32, 0u32);
        for b in word.bytes() {
            if b.is_ascii_lowercase() {
                lower |= 1 << (b - b'a');
            } else {
                upper |= 1 << (b - b'A');
            }
        }
        (lower & upper).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
    }

    #[test]
    fn test_all_uppercase() {
        assert_eq!(Solution::number_of_special_chars("ABC".to_string()), 0);
    }

    #[test]
    fn test_all_special() {
        assert_eq!(
            Solution::number_of_special_chars(
                "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ".to_string()
            ),
            26
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::number_of_special_chars("a".to_string()), 0);
        assert_eq!(Solution::number_of_special_chars("A".to_string()), 0);
    }
}
