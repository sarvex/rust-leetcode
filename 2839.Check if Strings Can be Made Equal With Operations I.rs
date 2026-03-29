impl Solution {
    /// Compares sorted character groups at even and odd indices.
    ///
    /// # Intuition
    /// Swapping at indices with difference 2 means characters at even positions
    /// (0, 2) can only interchange with each other, and characters at odd
    /// positions (1, 3) can only interchange with each other. Two strings are
    /// reachable from one another iff both groups contain the same multiset of
    /// characters.
    ///
    /// # Approach
    /// 1. Collect the bytes of each string.
    /// 2. Check that the sorted pair at even indices matches across both strings.
    /// 3. Check the same for odd indices.
    ///
    /// # Complexity
    /// - Time: O(1) — fixed-length strings of size 4.
    /// - Space: O(1)
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let a = s1.as_bytes();
        let b = s2.as_bytes();

        let sorted_pair = |s: &[u8], i: usize, j: usize| -> (u8, u8) {
            if s[i] <= s[j] {
                (s[i], s[j])
            } else {
                (s[j], s[i])
            }
        };

        sorted_pair(a, 0, 2) == sorted_pair(b, 0, 2) && sorted_pair(a, 1, 3) == sorted_pair(b, 1, 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swappable() {
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "cdab".to_string()
        ));
    }

    #[test]
    fn test_not_equal() {
        assert!(!Solution::can_be_equal(
            "abcd".to_string(),
            "dacb".to_string()
        ));
    }

    #[test]
    fn test_already_equal() {
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "abcd".to_string()
        ));
    }

    #[test]
    fn test_single_swap_even() {
        // Swap positions 0 and 2: "abcd" -> "cbad"
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "cbad".to_string()
        ));
    }

    #[test]
    fn test_single_swap_odd() {
        // Swap positions 1 and 3: "abcd" -> "adcb"
        assert!(Solution::can_be_equal(
            "abcd".to_string(),
            "adcb".to_string()
        ));
    }

    #[test]
    fn test_all_same() {
        assert!(Solution::can_be_equal(
            "aaaa".to_string(),
            "aaaa".to_string()
        ));
    }

    #[test]
    fn test_mismatch_odd_group() {
        // Even group matches (a,a) but odd group differs (b,d) vs (c,d)
        assert!(!Solution::can_be_equal(
            "abad".to_string(),
            "acad".to_string()
        ));
    }
}
