impl Solution {
    /// Finds the added character using XOR of all bytes.
    ///
    /// # Intuition
    /// XOR-ing all characters of both strings cancels out every pair, leaving
    /// only the extra character from `t`.
    ///
    /// # Approach
    /// 1. Chain the bytes of `s` and `t`.
    /// 2. Fold with XOR to isolate the unique byte.
    /// 3. Convert the result back to a char.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_the_difference(s: String, t: String) -> char {
        s.bytes()
            .chain(t.bytes())
            .fold(0u8, |acc, b| acc ^ b)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }

    #[test]
    fn test_empty_s() {
        assert_eq!(
            Solution::find_the_difference(String::new(), "y".to_string()),
            'y'
        );
    }

    #[test]
    fn test_same_chars() {
        assert_eq!(
            Solution::find_the_difference("a".to_string(), "aa".to_string()),
            'a'
        );
    }
}
