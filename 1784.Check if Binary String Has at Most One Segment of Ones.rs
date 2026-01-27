impl Solution {
    /// Checks that ones form a single contiguous segment (no "01" pattern).
    ///
    /// # Intuition
    /// If the string contains "01", then there is a zero between two segments
    /// of ones (since leading zeros are not allowed and it starts with '1').
    ///
    /// # Approach
    /// 1. Check if the substring "01" exists.
    /// 2. If not, all ones are contiguous.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_segment() {
        assert!(Solution::check_ones_segment("1001".to_string()) == false);
    }

    #[test]
    fn test_contiguous() {
        assert!(Solution::check_ones_segment("110".to_string()));
    }
}
