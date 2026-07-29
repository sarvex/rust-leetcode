impl Solution {
    /// Sort the first half of the palindrome to get the lexicographically smallest arrangement.
    ///
    /// # Intuition
    /// Since `s` is guaranteed palindromic, every character has an even count except at most
    /// one (the middle character in odd-length strings). The first half fully determines the
    /// palindrome. Sorting the first half in ascending order and mirroring it gives the
    /// lexicographically smallest result, with the odd middle character placed in the center.
    ///
    /// # Approach
    /// 1. Count frequency of each character.
    /// 2. Build the first half by iterating `'a'..='z'` and appending `freq[c] / 2` copies.
    /// 3. Find the middle character (the one with an odd count), if any.
    /// 4. Construct: first_half + middle + reversed(first_half).
    ///
    /// # Complexity
    /// - Time: O(n) — one pass to count, one pass to build (alphabet is constant 26)
    /// - Space: O(n) — output string
    pub fn smallest_palindrome(s: String) -> String {
        let mut freq = [0u32; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let half_len = s.len() / 2;
        let mut half = Vec::with_capacity(half_len);

        let mut middle = None;
        for (i, &count) in freq.iter().enumerate() {
            let ch = b'a' + i as u8;
            for _ in 0..count / 2 {
                half.push(ch);
            }
            if count % 2 == 1 {
                middle = Some(ch);
            }
        }

        // half is already sorted ascending because we iterate 'a'..'z'
        let mut result = Vec::with_capacity(s.len());
        result.extend_from_slice(&half);
        if let Some(m) = middle {
            result.push(m);
        }
        result.extend(half.iter().rev());

        // SAFETY: all bytes are valid ASCII lowercase letters
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::smallest_palindrome("z".to_string()), "z");
    }

    #[test]
    fn test_odd_length() {
        assert_eq!(Solution::smallest_palindrome("babab".to_string()), "abbba");
    }

    #[test]
    fn test_even_length() {
        assert_eq!(Solution::smallest_palindrome("daccad".to_string()), "acddca");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::smallest_palindrome("aaaa".to_string()), "aaaa");
    }

    #[test]
    fn test_two_chars_even() {
        // "abba" -> half = "ab", reversed = "ba" -> "abba"
        assert_eq!(Solution::smallest_palindrome("abba".to_string()), "abba");
    }

    #[test]
    fn test_two_chars_odd() {
        // "aabaa" -> half = "aa", middle = 'b' -> "aabaa"
        assert_eq!(Solution::smallest_palindrome("aabaa".to_string()), "aabaa");
    }

    #[test]
    fn test_already_smallest() {
        assert_eq!(Solution::smallest_palindrome("aaa".to_string()), "aaa");
    }
}
