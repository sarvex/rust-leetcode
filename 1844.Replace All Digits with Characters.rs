impl Solution {
    /// Replaces each digit with the character shifted from the preceding letter.
    ///
    /// # Intuition
    /// Odd-indexed positions contain digits that should become the character
    /// obtained by shifting the previous letter by that digit amount.
    ///
    /// # Approach
    /// 1. Convert the string to a mutable byte vector.
    /// 2. For each odd index, replace the digit with the previous byte plus the digit value.
    /// 3. Convert back to a string.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn replace_digits(s: String) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len();
        let mut i = 1;
        while i < n {
            bytes[i] = bytes[i - 1] + (bytes[i] - b'0');
            i += 2;
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        assert_eq!(Solution::replace_digits("a1c1e1".to_string()), "abcdef");
    }

    #[test]
    fn test_larger_shifts() {
        assert_eq!(
            Solution::replace_digits("a1b2c3d4e5".to_string()),
            "abbdcfdhe5"
                .to_string()
                .replace("5", &((b'e' + 5) as char).to_string())
        );
    }
}
