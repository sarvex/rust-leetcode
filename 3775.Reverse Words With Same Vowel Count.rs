impl Solution {
    /// Count vowels in the first word, then reverse each later word that has the same count.
    ///
    /// # Intuition
    /// The first word defines the "target" vowel count; only words with that count get reversed.
    ///
    /// # Approach
    /// Scan words by locating space boundaries in the byte buffer. Count vowels with a branchless
    /// bitmask test. Reverse matching words via stdlib `reverse()` which is SIMD-optimized.
    /// No sentinel byte or extra allocation needed.
    ///
    /// # Complexity
    /// - Time: O(n) over bytes
    /// - Space: O(1) extra beyond the input string
    pub fn reverse_words(s: String) -> String {
        const VOWEL_MASK: u32 = (1 << 0) | (1 << 4) | (1 << 8) | (1 << 14) | (1 << 20);

        #[inline(always)]
        fn vowel_count(word: &[u8]) -> usize {
            word.iter()
                .filter(|b| {
                    let idx = b.wrapping_sub(b'a');
                    idx < 26 && (VOWEL_MASK >> idx) & 1 == 1
                })
                .count()
        }

        let mut bytes = s.into_bytes();
        let len = bytes.len();
        let mut first_vowels = None;
        let mut pos = 0;

        while pos < len {
            if bytes[pos] == b' ' {
                pos += 1;
                continue;
            }
            let start = pos;
            while pos < len && bytes[pos] != b' ' {
                pos += 1;
            }
            let vc = vowel_count(&bytes[start..pos]);
            match first_vowels {
                None => first_vowels = Some(vc),
                Some(fv) if vc == fv => bytes[start..pos].reverse(),
                _ => {}
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::reverse_words("cat and mice".into()),
            "cat dna mice"
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::reverse_words("book is nice".into()),
            "book is ecin"
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::reverse_words("banana healthy".into()),
            "banana healthy"
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(Solution::reverse_words("hello".into()), "hello");
    }

    #[test]
    fn test_all_same_vowel_count() {
        assert_eq!(
            Solution::reverse_words("a e i".into()),
            "a e i"
        );
    }
}
