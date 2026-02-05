impl Solution {
    /// Computes the bitwise complement of a base-10 integer.
    ///
    /// # Intuition
    /// Flip each bit of n. The complement is `n XOR mask` where mask has
    /// all bits set up to the highest bit of n.
    ///
    /// # Approach
    /// Build a mask with the same bit length as n by left-shifting. Return
    /// `mask - 1 - n` which flips all significant bits.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut mask = 1;
        while mask <= n {
            mask <<= 1;
        }
        mask - 1 - n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!(Solution::bitwise_complement(5), 2); // 101 -> 010
    }

    #[test]
    fn test_seven() {
        assert_eq!(Solution::bitwise_complement(7), 0); // 111 -> 000
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::bitwise_complement(0), 1);
    }

    #[test]
    fn test_ten() {
        assert_eq!(Solution::bitwise_complement(10), 5); // 1010 -> 0101
    }
}
