use std::collections::VecDeque;

impl Solution {
    /// Finds the maximum in each sliding window using a monotonic deque.
    ///
    /// # Intuition
    /// Maintain a deque of indices in decreasing order of their values.
    /// The front always holds the index of the current window's maximum.
    ///
    /// # Approach
    /// 1. For each element, remove indices outside the window from the front.
    /// 2. Remove smaller elements from the back (they can never be the max).
    /// 3. Push the current index.
    /// 4. Once the window is full, record the front element as the maximum.
    ///
    /// # Complexity
    /// - Time: O(n) — each element is pushed and popped at most once
    /// - Space: O(k) for the deque
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);
        let mut deque: VecDeque<usize> = VecDeque::new();

        for i in 0..nums.len() {
            if let Some(&front) = deque.front() {
                if i >= front + k {
                    deque.pop_front();
                }
            }
            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
            if i >= k - 1 {
                result.push(nums[*deque.front().unwrap()]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn window_equals_array() {
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }
}
