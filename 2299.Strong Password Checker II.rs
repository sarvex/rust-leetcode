impl Solution {
    /// Validates a password against strong password criteria.
    ///
    /// # Intuition
    /// Track character categories via a bitmask while scanning for adjacent
    /// duplicates. All four categories (upper, lower, digit, special) must
    /// appear and no two adjacent characters may be identical.
    ///
    /// # Approach
    /// 1. Reject passwords shorter than 8 characters
    /// 2. Scan bytes, checking for adjacent duplicates and accumulating a 4-bit mask
    /// 3. Return true only if all 4 bits are set (mask == 0b1111)
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let bytes = password.as_bytes();
        let mut mask = 0u8;
        let mut prev = 0u8;

        for &c in bytes {
            if c == prev {
                return false;
            }
            mask |= match c {
                b'A'..=b'Z' => 0b1000,
                b'a'..=b'z' => 0b0100,
                b'0'..=b'9' => 0b0010,
                _ => 0b0001,
            };
            prev = c;
        }

        mask == 0b1111
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_password() {
        assert!(Solution::strong_password_checker_ii(
            "IloveLe3tcode!".to_string()
        ));
    }

    #[test]
    fn test_missing_special() {
        assert!(!Solution::strong_password_checker_ii(
            "Me+You--IsMyDream".to_string()
        ));
    }

    #[test]
    fn test_too_short() {
        assert!(!Solution::strong_password_checker_ii("Ab1!".to_string()));
    }

    #[test]
    fn test_adjacent_duplicates() {
        assert!(!Solution::strong_password_checker_ii(
            "AAbb11!!cc".to_string()
        ));
    }

    #[test]
    fn test_missing_digit() {
        assert!(!Solution::strong_password_checker_ii(
            "Abcdefgh!".to_string()
        ));
    }
}
