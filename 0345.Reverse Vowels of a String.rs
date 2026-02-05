impl Solution {
    /// Reverses only the vowels in a string using two pointers on a byte vector.
    ///
    /// # Intuition
    /// Use two pointers from both ends, skipping consonants and swapping vowels.
    ///
    /// # Approach
    /// 1. Convert string to a mutable `Vec<u8>`.
    /// 2. Advance left past consonants, retreat right past consonants.
    /// 3. Swap when both point to vowels.
    /// 4. Reconstruct the string from the modified bytes.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the byte vector
    pub fn reverse_vowels(s: String) -> String {
        let mut bytes = s.into_bytes();
        if bytes.len() <= 1 {
            return String::from_utf8(bytes).unwrap();
        }
        let is_vowel = |c: u8| matches!(c.to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u');
        let (mut left, mut right) = (0, bytes.len() - 1);
        while left < right {
            while left < right && !is_vowel(bytes[left]) {
                left += 1;
            }
            while left < right && !is_vowel(bytes[right]) {
                right -= 1;
            }
            if left < right {
                bytes.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
    }

    #[test]
    fn leetcode() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }

    #[test]
    fn all_vowels() {
        assert_eq!(
            Solution::reverse_vowels("aeiou".to_string()),
            "uoiea".to_string()
        );
    }

    #[test]
    fn no_vowels() {
        assert_eq!(
            Solution::reverse_vowels("bcdfg".to_string()),
            "bcdfg".to_string()
        );
    }

    #[test]
    fn single_char_vowel() {
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a".to_string());
    }

    #[test]
    fn single_char_consonant() {
        assert_eq!(Solution::reverse_vowels("b".to_string()), "b".to_string());
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::reverse_vowels(String::new()), String::new());
    }

    #[test]
    fn mixed_case_vowels() {
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string());
    }

    #[test]
    fn upper_case_vowels() {
        assert_eq!(
            Solution::reverse_vowels("HELLO".to_string()),
            "HOLLE".to_string()
        );
    }

    #[test]
    fn vowels_at_ends() {
        assert_eq!(
            Solution::reverse_vowels("aXe".to_string()),
            "eXa".to_string()
        );
    }
}
