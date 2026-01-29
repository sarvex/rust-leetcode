impl Solution {
    /// Counts minimum flips for an alternating binary string.
    ///
    /// # Intuition
    /// There are only two valid alternating patterns: "010101..." and "101010...".
    /// Count mismatches against one pattern; the other pattern's mismatches are
    /// `n - count`.
    ///
    /// # Approach
    /// 1. Count positions where `s[i]` differs from the pattern starting with '0'.
    /// 2. Return the minimum of that count and `n - count`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_operations(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mismatches = bytes
            .iter()
            .enumerate()
            .filter(|(i, b)| **b != if *i % 2 == 0 { b'0' } else { b'1' })
            .count();
        mismatches.min(n - mismatches) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_operations("0100".to_string()), 1);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::min_operations("10".to_string()), 0);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(Solution::min_operations("1111".to_string()), 2);
    }
}
