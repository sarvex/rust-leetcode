impl Solution {
    /// Greedy replacement of the first non-'a' character in the left half.
    ///
    /// # Intuition
    /// To break a palindrome into the lexicographically smallest non-palindrome,
    /// change the first character that isn't 'a' (in the left half) to 'a'.
    /// If the entire left half is 'a', change the last character to 'b'.
    ///
    /// # Approach
    /// 1. Return empty string for single-character palindromes
    /// 2. Find the first non-'a' character in the first half
    /// 3. If found, replace it with 'a'
    /// 4. If all left-half characters are 'a', replace the last character with 'b'
    ///
    /// # Complexity
    /// - Time: O(n) single scan of the first half
    /// - Space: O(n) for the mutable byte vector
    pub fn break_palindrome(palindrome: String) -> String {
        let n = palindrome.len();
        if n == 1 {
            return String::new();
        }

        let mut bytes = palindrome.into_bytes();

        match bytes[..n / 2].iter().position(|&b| b != b'a') {
            Some(i) => bytes[i] = b'a',
            None => *bytes.last_mut().unwrap() = b'b',
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_first_non_a() {
        assert_eq!(Solution::break_palindrome("abccba".to_string()), "aaccba");
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::break_palindrome("a".to_string()), "");
    }

    #[test]
    fn all_a_palindrome() {
        assert_eq!(Solution::break_palindrome("aaa".to_string()), "aab");
    }

    #[test]
    fn two_characters() {
        assert_eq!(Solution::break_palindrome("aa".to_string()), "ab");
    }
}
