impl Solution {
    /// Remove trailing zeros from a numeric string.
    ///
    /// # Intuition
    /// Trim from the right while the character is '0'.
    ///
    /// # Approach
    /// 1. Convert to bytes for efficient indexing.
    /// 2. Find the last non-zero byte position.
    /// 3. Slice up to that position (inclusive) and return as `String`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the returned string
    pub fn remove_trailing_zeros(num: String) -> String {
        let bytes = num.as_bytes();
        let end = bytes.iter().rposition(|&b| b != b'0').unwrap_or(0);
        num[..=end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_multiple_trailing_zeros() {
        assert_eq!(
            Solution::remove_trailing_zeros("51200400".to_string()),
            "512004"
        );
    }

    #[test]
    fn no_trailing_zeros_unchanged() {
        assert_eq!(Solution::remove_trailing_zeros("123".to_string()), "123");
    }
}
