use std::collections::VecDeque;

impl Solution {
    /// Sliding window with monotonic deque tracking stair-step structure
    ///
    /// # Intuition
    /// To make a subarray non-decreasing with minimum operations, each element must be
    /// incremented to at least the running maximum (prefix max). Process right-to-left
    /// so that extending the window leftward adds a new potential maximum that dominates
    /// subsequent elements.
    ///
    /// # Approach
    /// Process from right to left using two pointers. Maintain a monotonic deque of
    /// (value, count, sum) segments. When adding element at position l:
    /// - Merge segments where max <= nums[l] (these elements are now dominated by nums[l])
    /// - Update cost based on how much increment each dominated element needs
    /// - Shrink from right while cost exceeds k
    ///
    /// # Complexity
    /// - Time: O(n) - each element pushed/popped at most once
    /// - Space: O(n) - for the monotonic deque
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;

        let mut result = 0i64;
        let mut cost = 0i64;
        let mut r = n;

        // Pre-allocate deque: (max_value, count, sum)
        let mut dq: VecDeque<(i64, i64, i64)> = VecDeque::with_capacity(n);

        for l in (0..n).rev() {
            let val = nums[l] as i64;
            let mut cnt = 1i64;
            let mut sum = val;

            // Merge segments dominated by the new leftmost element
            while let Some(&(seg_max, seg_cnt, seg_sum)) = dq.back() {
                if seg_max <= val {
                    dq.pop_back();
                    cost -= seg_max * seg_cnt - seg_sum;
                    cnt += seg_cnt;
                    sum += seg_sum;
                } else {
                    break;
                }
            }

            cost += val * cnt - sum;
            dq.push_back((val, cnt, sum));

            // Shrink from right while cost exceeds k
            while cost > k && r > l {
                r -= 1;
                let (front_max, front_cnt, front_sum) = dq.front_mut().unwrap();
                let right_val = nums[r] as i64;
                cost -= *front_max - right_val;
                *front_cnt -= 1;
                *front_sum -= right_val;

                if *front_cnt == 0 {
                    dq.pop_front();
                }
            }

            // Handle edge case where window becomes invalid
            if r <= l {
                r = l + 1;
                dq.clear();
                dq.push_back((val, 1, val));
                cost = 0;
            }

            result += (r - l) as i64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7),
            17
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4),
            12
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_non_decreasing_subarrays(vec![5], 0), 1);
    }

    #[test]
    fn test_already_non_decreasing() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![1, 2, 3, 4], 0),
            10
        );
    }

    #[test]
    fn test_decreasing_array() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![4, 3, 2, 1], 10),
            10
        );
    }
}
