impl Solution {
    /// State-machine validation for determining if a string is a valid number.
    ///
    /// # Intuition
    /// A valid number follows a grammar with optional sign, digits, optional
    /// decimal point, and optional exponent with its own sign and digits.
    /// Tracking boolean flags for each component encountered validates the
    /// grammar in a single pass.
    ///
    /// # Approach
    /// Scan bytes sequentially. Track whether digits, a dot, or an exponent
    /// have been seen. Handle sign at the start or immediately after 'e'/'E'.
    /// Validate that digits appear before and after the exponent, and that
    /// a dot is not repeated or placed after an exponent.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the string
    /// - Space: O(1) — boolean flags only
    pub fn is_number(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut i = 0;

        if i < n && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }

        let mut has_digits = false;
        let mut has_dot = false;
        let mut has_exponent = false;

        while i < n {
            match bytes[i] {
                b'0'..=b'9' => has_digits = true,
                b'.' => {
                    if has_dot || has_exponent {
                        return false;
                    }
                    has_dot = true;
                }
                b'e' | b'E' => {
                    if has_exponent || !has_digits {
                        return false;
                    }
                    has_exponent = true;
                    has_digits = false;
                    if i + 1 < n && (bytes[i + 1] == b'+' || bytes[i + 1] == b'-') {
                        i += 1;
                    }
                }
                _ => return false,
            }
            i += 1;
        }

        has_digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_integer() {
        assert!(Solution::is_number("2".to_string()));
    }

    #[test]
    fn valid_decimal() {
        assert!(Solution::is_number("0089".to_string()));
    }

    #[test]
    fn valid_negative_decimal() {
        assert!(Solution::is_number("-0.1".to_string()));
    }

    #[test]
    fn valid_exponent() {
        assert!(Solution::is_number("2e10".to_string()));
    }

    #[test]
    fn valid_negative_exponent() {
        assert!(Solution::is_number("-90E3".to_string()));
    }

    #[test]
    fn invalid_alpha() {
        assert!(!Solution::is_number("abc".to_string()));
    }

    #[test]
    fn invalid_exponent_no_base() {
        assert!(!Solution::is_number("e3".to_string()));
    }

    #[test]
    fn invalid_double_dot() {
        assert!(!Solution::is_number("1.2.3".to_string()));
    }

    #[test]
    fn valid_dot_with_digits() {
        assert!(Solution::is_number(".1".to_string()));
    }

    #[test]
    fn invalid_lone_dot() {
        assert!(!Solution::is_number(".".to_string()));
    }
}
