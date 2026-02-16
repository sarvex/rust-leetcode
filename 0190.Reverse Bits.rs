impl Solution {
    /// Reverses the bits of a 32-bit unsigned integer.
    ///
    /// # Intuition
    /// Use divide and conquer to swap bits in parallel: first swap adjacent bits,
    /// then swap pairs, then nibbles, bytes, and finally half-words.
    ///
    /// # Approach
    /// 1. Swap odd and even bits
    /// 2. Swap consecutive pairs
    /// 3. Swap nibbles (4 bits)
    /// 4. Swap bytes
    /// 5. Swap half-words (16 bits)
    ///
    /// # Complexity
    /// - Time: O(1) — only 5 bitwise operations
    /// - Space: O(1)
    pub fn reverse_bits(n: i32) -> i32 {
        let mut n = n as u32;
        // Swap odd and even bits
        n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1);
        // Swap consecutive pairs
        n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2);
        // Swap nibbles
        n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4);
        // Swap bytes
        n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8);
        // Swap half-words (16 bits)
        n = (n >> 16) | (n << 16);
        n as i32
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
        assert_eq!(Solution::reverse_bits(-1), -1);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::reverse_bits(0), 0);
    }
}
