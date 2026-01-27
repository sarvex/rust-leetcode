use std::collections::VecDeque;

impl Solution {
    /// Counts non-decreasing subarrays achievable with at most k increment operations.
    ///
    /// # Intuition
    /// Processing right-to-left, each new left element may dominate subsequent
    /// elements. A monotonic deque of (value, count, sum) segments tracks the
    /// staircase structure; merging dominated segments updates the total cost.
    ///
    /// # Approach
    /// 1. Sweep l from n-1 to 0; maintain a right boundary r and a deque.
    /// 2. When adding nums[l], merge back-segments whose max ≤ nums[l].
    /// 3. Shrink from the front (right end) while cost exceeds k.
    /// 4. Accumulate (r − l) into the answer for each l.
    ///
    /// # Complexity
    /// - Time: O(n) — each element pushed and popped at most once
    /// - Space: O(n) for the deque
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;

        let mut result = 0i64;
        let mut cost = 0i64;
        let mut r = n;

        let mut dq: VecDeque<(i64, i64, i64)> = VecDeque::with_capacity(n);

        for l in (0..n).rev() {
            let val = nums[l] as i64;
            let mut cnt = 1i64;
            let mut sum = val;

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
    fn mixed_array_with_budget() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7),
            17
        );
    }

    #[test]
    fn another_mixed_example() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4),
            12
        );
    }

    #[test]
    fn single_element_always_valid() {
        assert_eq!(Solution::count_non_decreasing_subarrays(vec![5], 0), 1);
    }

    #[test]
    fn already_non_decreasing_counts_all() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![1, 2, 3, 4], 0),
            10
        );
    }

    #[test]
    fn decreasing_with_sufficient_budget() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![4, 3, 2, 1], 10),
            10
        );
    }
}
