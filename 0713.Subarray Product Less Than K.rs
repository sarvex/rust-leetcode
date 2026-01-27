impl Solution {
    /// Counts subarrays with product less than k using sliding window.
    ///
    /// # Intuition
    /// A sliding window maintains the product of elements in the current range.
    /// Each new right endpoint contributes `(r - l + 1)` new valid subarrays.
    ///
    /// # Approach
    /// Expand the window by multiplying the right element. Shrink from the left
    /// when the product reaches or exceeds k. The count of valid subarrays
    /// ending at each right index is the window size.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let mut count = 0;
        let mut product = 1;
        let mut left = 0;
        for (right, &x) in nums.iter().enumerate() {
            product *= x;
            while product >= k {
                product /= nums[left];
                left += 1;
            }
            count += (right - left + 1) as i32;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }

    #[test]
    fn test_no_valid_subarray() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 2),
            6
        );
    }

    #[test]
    fn test_single_element_valid() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![3], 4), 1);
    }

    #[test]
    fn test_single_element_invalid() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![3], 3), 0);
    }
}
