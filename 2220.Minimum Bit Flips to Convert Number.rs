impl Solution {
    /// Count the minimum bit flips to convert one number to another.
    ///
    /// # Intuition
    /// XOR reveals differing bits; counting set bits in the XOR gives the answer.
    ///
    /// # Approach
    /// Compute `start ^ goal` and count its set bits via `count_ones()`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
    }

    #[test]
    fn same_number() {
        assert_eq!(Solution::min_bit_flips(5, 5), 0);
    }

    #[test]
    fn zero_to_max() {
        assert_eq!(Solution::min_bit_flips(0, 0b1111), 4);
    }
}
