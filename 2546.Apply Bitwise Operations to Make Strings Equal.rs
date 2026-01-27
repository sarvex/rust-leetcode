impl Solution {
    /// Determines if string s can be transformed into target using OR/XOR operations.
    ///
    /// # Intuition
    /// The operations can freely rearrange 1-bits but cannot create a 1 from all zeros
    /// or eliminate all 1-bits. Thus the only invariant is whether any '1' exists.
    ///
    /// # Approach
    /// Both strings must either both contain '1' or both be all '0'.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.as_bytes().iter().any(|&b| b == b'1') == target.as_bytes().iter().any(|&b| b == b'1')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_have_ones() {
        assert!(Solution::make_strings_equal(
            "1010".to_string(),
            "0110".to_string()
        ));
    }

    #[test]
    fn test_target_all_zeros() {
        assert!(!Solution::make_strings_equal(
            "11".to_string(),
            "00".to_string()
        ));
    }

    #[test]
    fn test_both_all_zeros() {
        assert!(Solution::make_strings_equal(
            "00".to_string(),
            "00".to_string()
        ));
    }

    #[test]
    fn test_source_all_zeros() {
        assert!(!Solution::make_strings_equal(
            "00".to_string(),
            "01".to_string()
        ));
    }
}
