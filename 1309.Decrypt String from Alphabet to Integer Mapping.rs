impl Solution {
    /// Reverse scan decoding alphabet-to-integer mapping.
    ///
    /// # Intuition
    /// Scanning from the end simplifies detecting `#`-suffixed two-digit codes.
    /// When `#` is found, consume two preceding digits as a 10–26 mapping;
    /// otherwise consume one digit as a 1–9 mapping.
    ///
    /// # Approach
    /// 1. Scan from left, checking two positions ahead for `#`
    /// 2. If `#` found: parse two-digit number, advance by 3
    /// 3. Otherwise: parse single digit, advance by 1
    /// 4. Map number to character via `(96 + num) as char`
    ///
    /// # Complexity
    /// - Time: O(n) single pass
    /// - Space: O(n) for the output string
    pub fn freq_alphabets(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut result = String::with_capacity(n);
        let mut i = 0;

        while i < n {
            let num = if i + 2 < n && bytes[i + 2] == b'#' {
                let v = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                i += 3;
                v
            } else {
                let v = bytes[i] - b'0';
                i += 1;
                v
            };
            result.push((96 + num) as char);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_encoding() {
        assert_eq!(Solution::freq_alphabets("10#11#12".to_string()), "jkab");
    }

    #[test]
    fn all_single_digits() {
        assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz");
    }

    #[test]
    fn all_hash_encoded() {
        assert_eq!(Solution::freq_alphabets("25#26#".to_string()), "yz");
    }
}
