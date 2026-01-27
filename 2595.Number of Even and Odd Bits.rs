impl Solution {
    /// Count set bits at even and odd positions.
    ///
    /// # Intuition
    /// Iterate through bits, toggling between even and odd position counters.
    ///
    /// # Approach
    /// 1. Extract bits one by one with right shifts
    /// 2. Toggle index between 0 (even) and 1 (odd) using XOR
    /// 3. Accumulate counts
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut ans = vec![0; 2];
        let mut i = 0;

        while n != 0 {
            ans[i] += n & 1;
            n >>= 1;
            i ^= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_17() {
        assert_eq!(Solution::even_odd_bit(17), vec![2, 0]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::even_odd_bit(0), vec![0, 0]);
    }

    #[test]
    fn test_all_bits() {
        assert_eq!(Solution::even_odd_bit(7), vec![2, 1]);
    }
}
