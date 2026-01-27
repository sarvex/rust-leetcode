impl Solution {
    /// Finds a valid subarray size using monotonic stack boundaries.
    ///
    /// # Intuition
    /// For a subarray of length k to be valid, every element must exceed threshold/k.
    /// The bottleneck is the minimum element — if min * k > threshold, all elements
    /// satisfy the condition. Use monotonic stacks to find each element's maximum
    /// range where it is the minimum.
    ///
    /// # Approach
    /// 1. Find nearest smaller element to the left for each index
    /// 2. Find nearest smaller element to the right for each index
    /// 3. For each element as potential minimum, check if element * range > threshold
    ///
    /// # Complexity
    /// - Time: O(n) — each element pushed/popped from stack at most once
    /// - Space: O(n) — for left/right boundary arrays and stack
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let threshold = threshold as i64;

        let mut left = vec![-1_i64; n];
        let mut right = vec![n as i64; n];
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            while stack.last().is_some_and(|&top| nums[top] >= nums[i]) {
                stack.pop();
            }
            left[i] = stack.last().map_or(-1, |&x| x as i64);
            stack.push(i);
        }

        stack.clear();

        for i in (0..n).rev() {
            while stack.last().is_some_and(|&top| nums[top] >= nums[i]) {
                stack.pop();
            }
            right[i] = stack.last().map_or(n as i64, |&x| x as i64);
            stack.push(i);
        }

        (0..n)
            .find(|&i| {
                let k = right[i] - left[i] - 1;
                nums[i] as i64 * k > threshold
            })
            .map_or(-1, |i| (right[i] - left[i] - 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::valid_subarray_size(vec![1, 3, 4, 3, 1], 6), 3);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::valid_subarray_size(vec![6, 5, 6, 5, 8], 7);
        assert!(result >= 1 && result <= 5);
    }

    #[test]
    fn test_single_element_valid() {
        assert_eq!(Solution::valid_subarray_size(vec![10], 5), 1);
    }

    #[test]
    fn test_single_element_invalid() {
        assert_eq!(Solution::valid_subarray_size(vec![5], 10), -1);
    }

    #[test]
    fn test_no_valid_subarray() {
        assert_eq!(Solution::valid_subarray_size(vec![1, 1, 1, 1], 10), -1);
    }

    #[test]
    fn test_all_same_elements() {
        let result = Solution::valid_subarray_size(vec![5, 5, 5, 5], 10);
        assert!(result > 0);
    }

    #[test]
    fn test_boundary_equal() {
        assert_eq!(
            Solution::valid_subarray_size(vec![1_000_000_000], 1_000_000_000),
            -1
        );
    }

    #[test]
    fn test_increasing_sequence() {
        let result = Solution::valid_subarray_size(vec![1, 2, 3, 4, 5], 4);
        assert!(result > 0);
    }
}
