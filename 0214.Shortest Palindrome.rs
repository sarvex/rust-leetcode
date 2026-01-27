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
        for (i, c) in s.chars().enumerate() {
            let t = (c as u64) - (b'0' as u64) + 1;
            prefix = prefix.wrapping_mul(base).wrapping_add(t);
            suffix = suffix.wrapping_add(t.wrapping_mul(mul));
            mul = mul.wrapping_mul(base);
            if prefix == suffix {
                idx = i + 1;
            }
        }
        if idx == s.len() {
            s
        } else {
            let reversed: String = s[idx..].chars().rev().collect();
            reversed + &s
        }
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
