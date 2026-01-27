impl Solution {
    /// Returns the sign of the product of an array without overflow.
    ///
    /// # Intuition
    /// Only the sign matters — count negatives and check for zeros.
    /// Each negative flips the sign.
    ///
    /// # Approach
    /// 1. Fold over the array with a sign accumulator.
    /// 2. If any element is zero, the product is zero.
    /// 3. Otherwise, negate the accumulator for each negative element.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |sign, &num| match num.signum() {
            0 => 0,
            s => sign * s,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_product() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_contains_zero() {
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
    }

    #[test]
    fn test_negative_product() {
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
