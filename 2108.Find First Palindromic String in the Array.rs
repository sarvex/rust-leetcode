impl Solution {
    /// Finds the first palindromic string in the array using byte comparison.
    ///
    /// # Intuition
    /// A string is a palindrome if it reads the same forwards and backwards.
    /// Comparing bytes is more efficient than collecting reversed chars into
    /// a new string.
    ///
    /// # Approach
    /// 1. Iterate through the words.
    /// 2. For each word, check if its bytes match their reverse.
    /// 3. Return the first match, or an empty string if none found.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is the number of words, m is average word length
    /// - Space: O(1) extra
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| {
                let bytes = w.as_bytes();
                bytes.iter().eq(bytes.iter().rev())
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finds_palindrome() {
        let words = vec!["abc", "car", "ada", "racecar", "cool"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::first_palindrome(words), "ada");
    }

    #[test]
    fn test_no_palindrome() {
        let words = vec!["abc", "def"].into_iter().map(String::from).collect();
        assert_eq!(Solution::first_palindrome(words), "");
    }

    #[test]
    fn test_single_char_palindrome() {
        let words = vec!["a"].into_iter().map(String::from).collect();
        assert_eq!(Solution::first_palindrome(words), "a");
    }
}
