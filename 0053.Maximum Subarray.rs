impl Solution {
    /// Kadane's algorithm for maximum contiguous subarray sum.
    ///
    /// # Intuition
    /// At each position, the best subarray ending here either extends the
    /// previous best or starts fresh from the current element. This local
    /// decision propagates globally via a running maximum.
    ///
    /// # Approach
    /// Fold over the array maintaining two accumulators: `current_sum` (the
    /// maximum subarray sum ending at the current position) and `max_sum`
    /// (the global best). At each step, `current_sum` is the greater of
    /// the element alone or the element appended to the previous subarray.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the array
    /// - Space: O(1) — two scalar accumulators
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold((nums[0], nums[0]), |(current_sum, max_sum), &num| {
                let current_sum = current_sum.max(0) + num;
                (current_sum, max_sum.max(current_sum))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_values() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn all_positive() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn all_negative() {
        assert_eq!(Solution::max_sub_array(vec![-3, -2, -5, -1]), -1);
    }

    #[test]
    fn single_negative() {
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
