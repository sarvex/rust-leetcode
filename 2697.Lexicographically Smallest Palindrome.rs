impl Solution {
    /// Make the lexicographically smallest palindrome by changing characters.
    ///
    /// # Intuition
    /// For each mirrored pair, choose the smaller character for both positions.
    /// This ensures the result is a palindrome and lexicographically smallest.
    ///
    /// # Approach
    /// 1. Convert to mutable byte array
    /// 2. For each pair (i, n-1-i), set both to the minimum of the two
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut bytes: Vec<u8> = s.into_bytes();
        let n = bytes.len();

        (0..n / 2).for_each(|i| {
            let j = n - 1 - i;
            let min_char = bytes[i].min(bytes[j]);
            bytes[i] = min_char;
            bytes[j] = min_char;
        });

        String::from_utf8(bytes).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::make_smallest_palindrome("egcfe".into()), "efcfe");
    }

    #[test]
    fn test_already_palindrome() {
        assert_eq!(Solution::make_smallest_palindrome("abcba".into()), "abcba");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::make_smallest_palindrome("z".into()), "z");
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(Solution::make_smallest_palindrome("ba".into()), "aa");
    }
}
