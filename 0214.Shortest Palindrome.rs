impl Solution {
    /// Finds the shortest palindrome by prepending characters using rolling hash.
    ///
    /// # Intuition
    /// Find the longest palindromic prefix of s. The remaining suffix, reversed
    /// and prepended, gives the shortest palindrome.
    ///
    /// # Approach
    /// 1. Use rolling hash to compute forward and backward hashes simultaneously.
    /// 2. When hashes match, record the palindromic prefix length.
    /// 3. Reverse the non-palindromic suffix and prepend it.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result string
    pub fn shortest_palindrome(s: String) -> String {
        let base: u64 = 131;
        let (mut idx, mut prefix, mut suffix, mut mul) = (0usize, 0u64, 0u64, 1u64);
        for (byte_idx, c) in s.char_indices() {
            let t = (c as u64) + 1;
            prefix = prefix.wrapping_mul(base).wrapping_add(t);
            suffix = suffix.wrapping_add(t.wrapping_mul(mul));
            mul = mul.wrapping_mul(base);
            if prefix == suffix {
                idx = byte_idx + c.len_utf8();
            }
        }
        let s_len = s.len();
        if idx == s_len {
            return s;
        }
        let suffix_len = s_len - idx;
        let mut result = String::with_capacity(s_len + suffix_len);
        result.extend(s[idx..].chars().rev());
        result.push_str(&s);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa"
        );
    }

    #[test]
    fn prepend_needed() {
        assert_eq!(Solution::shortest_palindrome("abcd".to_string()), "dcbabcd");
    }

    #[test]
    fn already_palindrome() {
        assert_eq!(Solution::shortest_palindrome("aba".to_string()), "aba");
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::shortest_palindrome("".to_string()), "");
    }
}
