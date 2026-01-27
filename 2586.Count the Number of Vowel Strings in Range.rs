impl Solution {
    /// Count strings in range that start and end with a vowel.
    ///
    /// # Intuition
    /// Check each string's first and last bytes against the vowel set.
    ///
    /// # Approach
    /// 1. Slice the words from left to right
    /// 2. For each word, check first and last bytes against vowels
    /// 3. Count matches
    ///
    /// # Complexity
    /// - Time: O(right - left)
    /// - Space: O(1)
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let is_vowel = |c: u8| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u');

        words[left as usize..=right as usize]
            .iter()
            .filter(|w| {
                let b = w.as_bytes();
                is_vowel(b[0]) && is_vowel(b[b.len() - 1])
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_strings(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(String::from).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::vowel_strings(to_strings(vec!["are", "amy", "u"]), 0, 2),
            2
        );
    }

    #[test]
    fn test_no_vowels() {
        assert_eq!(
            Solution::vowel_strings(to_strings(vec!["hey", "aei", "bcd"]), 0, 2),
            1
        );
    }

    #[test]
    fn test_single_char_vowel() {
        assert_eq!(
            Solution::vowel_strings(to_strings(vec!["a", "e", "x"]), 0, 2),
            2
        );
    }
}
