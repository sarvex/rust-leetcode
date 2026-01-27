impl Solution {
    /// Checks if a number survives double digit-reversal.
    ///
    /// # Intuition
    /// Reversing a number twice restores the original unless trailing zeros
    /// were removed during the first reversal. Only zero itself and numbers
    /// without trailing zeros survive.
    ///
    /// # Approach
    /// 1. Return true if the number is zero (special case).
    /// 2. Otherwise return true only if the number has no trailing zeros.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert!(Solution::is_same_after_reversals(0));
    }

    #[test]
    fn test_no_trailing_zeros() {
        assert!(Solution::is_same_after_reversals(526));
    }

    #[test]
    fn test_trailing_zero() {
        assert!(!Solution::is_same_after_reversals(1800));
    }

    #[test]
    fn test_single_digit() {
        assert!(Solution::is_same_after_reversals(7));
    }
}
