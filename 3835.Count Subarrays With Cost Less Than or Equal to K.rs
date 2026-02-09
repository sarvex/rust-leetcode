use std::collections::VecDeque;

impl Solution {
    /// Sliding window with monotonic deques tracking window max and min.
    ///
    /// # Intuition
    /// For a fixed right endpoint, cost = (max - min) * length is non-decreasing as the
    /// window grows. So for each right there is a leftmost left with cost <= k.
    ///
    /// # Approach
    /// Maintain monotonic decreasing deque for max and increasing deque for min.
    /// For each right, expand window, then shrink from the left while cost > k.
    /// Pop deque fronts when they leave the window before computing cost.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut max_q = VecDeque::with_capacity(n);
        let mut min_q = VecDeque::with_capacity(n);
        let mut left = 0usize;
        let mut ans = 0i64;

        for right in 0..n {
            while max_q.back().is_some_and(|&i| nums[i] <= nums[right]) {
                max_q.pop_back();
            }
            max_q.push_back(right);

            while min_q.back().is_some_and(|&i| nums[i] >= nums[right]) {
                min_q.pop_back();
            }
            min_q.push_back(right);

            while (nums[*max_q.front().unwrap()] as i64 - nums[*min_q.front().unwrap()] as i64)
                * (right - left + 1) as i64
                > k
            {
                if max_q.front() == Some(&left) {
                    max_q.pop_front();
                }
                if min_q.front() == Some(&left) {
                    min_q.pop_front();
                }
                left += 1;
            }

            ans += (right - left + 1) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2], 4), 5);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::count_subarrays(vec![5, 5, 5, 5], 0), 10);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 0), 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_subarrays(vec![7], 0), 1);
        assert_eq!(Solution::count_subarrays(vec![7], 100), 1);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::count_subarrays(vec![1, 2], 0), 2);
        assert_eq!(Solution::count_subarrays(vec![1, 2], 2), 3);
    }
}
