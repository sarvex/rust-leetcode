impl Solution {
    /// Checks if a number has alternating bits.
    ///
    /// # Intuition
    /// If bits alternate, n XOR (n >> 1) produces all 1s. Adding 1 to an
    /// all-1s number yields a power of 2, which ANDed with itself minus 1 is 0.
    ///
    /// # Approach
    /// 1. Compute x = n ^ (n >> 1).
    /// 2. Check if x & (x + 1) == 0 (all bits are 1).
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = n as u32;
        let x = n ^ (n >> 1);
        x & (x + 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating() {
        assert!(Solution::has_alternating_bits(5));
        assert!(Solution::has_alternating_bits(10));
    }

    #[test]
    fn test_not_alternating() {
        assert!(!Solution::has_alternating_bits(7));
        assert!(!Solution::has_alternating_bits(11));
    }
}
