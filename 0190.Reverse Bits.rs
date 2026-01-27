impl Solution {
    /// Reverses the bits of a 32-bit unsigned integer.
    ///
    /// # Intuition
    /// Extract each bit from the least significant end and place it at the
    /// corresponding position from the most significant end.
    ///
    /// # Approach
    /// 1. Iterate through all 32 bit positions.
    /// 2. Extract the lowest bit of n and shift it to position `31 - i`.
    /// 3. OR into the result and right-shift n.
    ///
    /// # Complexity
    /// - Time: O(1) — always 32 iterations
    /// - Space: O(1)
    pub fn reverse_bits(mut n: u32) -> u32 {
        (0..32).fold(0u32, |ans, i| {
            let bit = (n & 1) << (31 - i);
            n >>= 1;
            ans | bit
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::reverse_bits(u32::MAX), u32::MAX);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::reverse_bits(0), 0);
    }
}
