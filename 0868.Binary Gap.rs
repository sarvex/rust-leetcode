impl Solution {
    /// Finds the longest distance between consecutive 1s in binary representation.
    ///
    /// # Intuition
    /// Track the position of the last seen 1-bit and compute the gap when
    /// the next 1-bit is found.
    ///
    /// # Approach
    /// Iterate through bits. When a 1-bit is encountered, update the maximum
    /// gap from the previous 1-bit position. Shift right until zero.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut max_gap = 0;
        let mut last_one = -1;
        let mut pos = 0;
        while n > 0 {
            if n & 1 == 1 {
                if last_one >= 0 {
                    max_gap = max_gap.max(pos - last_one);
                }
                last_one = pos;
            }
            pos += 1;
            n >>= 1;
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::binary_gap(22), 2); // 10110
    }

    #[test]
    fn test_8() {
        assert_eq!(Solution::binary_gap(8), 0); // 1000
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::binary_gap(5), 2); // 101
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::binary_gap(6), 1); // 110
    }
}
