impl Solution {
    /// Constructs minimum bitwise array using trailing ones analysis.
    ///
    /// # Intuition
    /// For `ans OR (ans+1)` to equal `num`, adding 1 flips the lowest 0 bit.
    /// The answer is `num` with the bit just below the lowest 0 bit cleared.
    ///
    /// # Approach
    /// 1. If `num` is 2, return -1 (no valid ans exists for even primes).
    /// 2. Count trailing ones to locate the carry boundary.
    /// 3. Clear the bit at `trailing_ones - 1` to produce the minimum ans.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of nums
    /// - Space: O(n) for the result array
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&num| match num {
                2 => -1,
                _ => num - (1 << (num.trailing_ones() - 1)),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_mixed_primes_with_even() {
        assert_eq!(
            Solution::min_bitwise_array(vec![2, 3, 5, 7]),
            vec![-1, 1, 4, 3]
        );
    }

    #[test]
    fn handles_larger_odd_primes() {
        assert_eq!(
            Solution::min_bitwise_array(vec![11, 13, 31]),
            vec![9, 12, 15]
        );
    }

    #[test]
    fn handles_single_element() {
        assert_eq!(Solution::min_bitwise_array(vec![3]), vec![1]);
    }
}
