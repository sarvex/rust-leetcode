impl Solution {
    /// Squares a sorted array and returns a sorted result using two pointers.
    ///
    /// # Intuition
    /// The largest squares come from the extremes of the sorted array.
    /// Two pointers from both ends fill the result from largest to smallest.
    ///
    /// # Approach
    /// Compare absolute values at both ends. Place the larger square at the
    /// back of the result and advance the corresponding pointer.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let (mut lo, mut hi) = (0, n - 1);
        for k in (0..n).rev() {
            if nums[lo].abs() > nums[hi].abs() {
                result[k] = nums[lo] * nums[lo];
                lo += 1;
            } else {
                result[k] = nums[hi] * nums[hi];
                hi = hi.wrapping_sub(1);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_signs() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test_negative_only() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
    }
}
