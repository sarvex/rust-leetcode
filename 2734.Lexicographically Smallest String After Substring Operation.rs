impl Solution {
    /// Lexicographically smallest string after decrementing one non-'a' substring.
    ///
    /// # Intuition
    /// Decrementing a contiguous block of non-'a' characters always produces a
    /// lexicographically smaller result. Skip leading 'a's, then decrement until
    /// hitting another 'a' or the end. If the entire string is 'a's, decrement
    /// the last character to 'z'.
    ///
    /// # Approach
    /// 1. Convert string to mutable bytes.
    /// 2. Skip leading 'a' bytes.
    /// 3. If all are 'a', set last byte to 'z' and return.
    /// 4. Otherwise, decrement consecutive non-'a' bytes by 1.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the byte buffer
    pub fn smallest_string(s: String) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len();
        let start = bytes.iter().position(|&b| b != b'a').unwrap_or(n);
        if start == n {
            bytes[n - 1] = b'z';
        } else {
            bytes[start..]
                .iter_mut()
                .take_while(|b| **b != b'a')
                .for_each(|b| *b -= 1);
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrement_middle_segment() {
        assert_eq!(Solution::smallest_string("cbabc".to_string()), "baabc");
    }

    #[test]
    fn all_a_wraps_last_to_z() {
        assert_eq!(Solution::smallest_string("aaa".to_string()), "aaz");
    }

    #[test]
    fn leading_a_skipped() {
        assert_eq!(Solution::smallest_string("acbbc".to_string()), "abaac");
    }
}
