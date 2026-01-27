impl Solution {
    /// Inserts digit x to maximize the numeric string value.
    ///
    /// # Intuition
    /// For positive numbers, insert x before the first digit smaller than x.
    /// For negative numbers, insert x before the first digit larger than x.
    ///
    /// # Approach
    /// 1. Determine if the number is negative.
    /// 2. Scan for the optimal insertion point.
    /// 3. Build the result by concatenating the prefix, digit, and suffix.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn max_value(n: String, x: i32) -> String {
        let bytes = n.as_bytes();
        let x_byte = b'0' + x as u8;
        let start = if bytes[0] == b'-' { 1 } else { 0 };

        let pos = if bytes[0] == b'-' {
            bytes[start..].iter().position(|&b| b > x_byte)
        } else {
            bytes[start..].iter().position(|&b| b < x_byte)
        };

        let insert_at = match pos {
            Some(p) => start + p,
            None => bytes.len(),
        };

        let mut result = String::with_capacity(n.len() + 1);
        result.push_str(&n[..insert_at]);
        result.push_str(&x.to_string());
        result.push_str(&n[insert_at..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_insert() {
        assert_eq!(Solution::max_value("99".to_string(), 9), "999");
    }

    #[test]
    fn test_negative_insert() {
        assert_eq!(Solution::max_value("-13".to_string(), 2), "-123");
    }

    #[test]
    fn test_insert_at_beginning() {
        assert_eq!(Solution::max_value("123".to_string(), 5), "5123");
    }
}
