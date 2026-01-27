impl Solution {
    /// Counts the number of set bits (Hamming weight) using the built-in method.
    ///
    /// # Intuition
    /// Rust's `count_ones()` efficiently counts set bits using hardware instructions
    /// when available.
    ///
    /// # Approach
    /// Delegate to `u32::count_ones()`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_set_bits() {
        assert_eq!(Solution::hammingWeight(0b1011), 3);
    }

    #[test]
    fn one_set_bit() {
        assert_eq!(Solution::hammingWeight(128), 1);
    }

    #[test]
    fn all_set_bits() {
        assert_eq!(Solution::hammingWeight(u32::MAX), 32);
    }
}
