impl Solution {
    /// Subarray With Elements Greater Than Varying Threshold
    ///
    /// # Intuition
    /// For a subarray of length k to be valid, every element must be > threshold/k.
    /// The bottleneck is the minimum element - if the minimum satisfies the condition,
    /// all elements do. So we need: min_element > threshold/k, or min_element * k > threshold.
    ///
    /// # Approach
    /// Use monotonic stack to find for each element the maximum subarray where it's the minimum:
    /// 1. Find nearest smaller element to the left for each index
    /// 2. Find nearest smaller element to the right for each index
    /// 3. For each element as potential minimum, calculate max subarray length
    /// 4. Check if element * length > threshold
    ///
    /// # Complexity
    /// - Time: O(n) - each element pushed/popped from stack at most once
    /// - Space: O(n) - for left/right boundary arrays and stack
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let threshold = threshold as i64;

        let mut left = vec![-1_i64; n];
        let mut right = vec![n as i64; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        // Find nearest smaller element to the left
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if nums[top] >= nums[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            left[i] = stack.last().map_or(-1, |&x| x as i64);
            stack.push(i);
        }

        stack.clear();

        // Find nearest smaller element to the right
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if nums[top] >= nums[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            right[i] = stack.last().map_or(n as i64, |&x| x as i64);
            stack.push(i);
        }

        // Check each element as the minimum of its maximum subarray
        for i in 0..n {
            let k = right[i] - left[i] - 1;
            let num = nums[i] as i64;

            // Valid if: num > threshold / k, equivalent to num * k > threshold
            if num * k > threshold {
                return k as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 3, 4, 3, 1];
        let threshold = 6;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![6, 5, 6, 5, 8];
        let threshold = 7;
        let result = Solution::valid_subarray_size(nums, threshold);
        // Valid answers are 1, 2, 3, 4, or 5
        assert!(result >= 1 && result <= 5);
    }

    #[test]
    fn test_single_element_valid() {
        let nums = vec![10];
        let threshold = 5;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), 1);
    }

    #[test]
    fn test_single_element_invalid() {
        let nums = vec![5];
        let threshold = 10;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), -1);
    }

    #[test]
    fn test_no_valid_subarray() {
        let nums = vec![1, 1, 1, 1];
        let threshold = 10;
        assert_eq!(Solution::valid_subarray_size(nums, threshold), -1);
    }

    #[test]
    fn test_all_same_elements() {
        let nums = vec![5, 5, 5, 5];
        let threshold = 10;
        // k=4, each element=5, 5*4=20 > 10, valid
        let result = Solution::valid_subarray_size(nums, threshold);
        assert!(result > 0);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![1_000_000_000];
        let threshold = 1_000_000_000;
        // 10^9 * 1 > 10^9 is false, so -1
        assert_eq!(Solution::valid_subarray_size(nums, threshold), -1);
    }

    #[test]
    fn test_increasing_sequence() {
        let nums = vec![1, 2, 3, 4, 5];
        let threshold = 4;
        let result = Solution::valid_subarray_size(nums, threshold);
        // Element 5 with k=1: 5 > 4, valid
        assert!(result > 0);
    }
}
