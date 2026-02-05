impl Solution {
    /// Check if string is already a palindrome for 1 or 2 removal answer.
    ///
    /// # Intuition
    /// The string contains only 'a' and 'b'. We can always remove all 'a's in
    /// one step and all 'b's in another (each forms a palindromic subsequence).
    /// If the string is already a palindrome, one removal suffices.
    ///
    /// # Approach
    /// 1. Check if the string equals its reverse
    /// 2. Return 1 if palindrome, 2 otherwise
    ///
    /// # Complexity
    /// - Time: O(n) for palindrome check
    /// - Space: O(1) with two-pointer check
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let is_palindrome = (0..n / 2).all(|i| bytes[i] == bytes[n - 1 - i]);
        if is_palindrome { 1 } else { 2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_palindrome() {
        assert_eq!(Solution::remove_palindrome_sub("abacaba".to_string()), 1);
    }

    #[test]
    fn not_palindrome() {
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::remove_palindrome_sub("bbbb".to_string()), 1);
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::remove_palindrome_sub("a".to_string()), 1);
    }
}
