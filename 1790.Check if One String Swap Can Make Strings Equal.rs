impl Solution {
    /// Checks if strings can be made equal with at most one swap.
    ///
    /// # Intuition
    /// Two strings are one-swap-equal if they differ at exactly zero or two
    /// positions, and swapping the mismatched characters makes them identical.
    ///
    /// # Approach
    /// 1. Collect indices where characters differ.
    /// 2. If no differences, strings are already equal.
    /// 3. If exactly two differences, verify cross-matching.
    /// 4. Otherwise, return false.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let diffs: Vec<usize> = (0..b1.len()).filter(|&i| b1[i] != b2[i]).collect();

        match diffs.len() {
            0 => true,
            2 => b1[diffs[0]] == b2[diffs[1]] && b1[diffs[1]] == b2[diffs[0]],
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_swap_needed() {
        assert!(Solution::are_almost_equal(
            "bank".to_string(),
            "kanb".to_string()
        ));
    }

    #[test]
    fn test_already_equal() {
        assert!(Solution::are_almost_equal(
            "abcd".to_string(),
            "abcd".to_string()
        ));
    }

    #[test]
    fn test_impossible() {
        assert!(!Solution::are_almost_equal(
            "abcd".to_string(),
            "dcba".to_string()
        ));
    }

    #[test]
    fn test_single_char() {
        assert!(Solution::are_almost_equal("a".to_string(), "a".to_string()));
    }
}
