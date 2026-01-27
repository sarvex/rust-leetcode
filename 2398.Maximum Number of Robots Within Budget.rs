use std::collections::VecDeque;

impl Solution {
    /// Finds maximum robots within budget using sliding window with monotonic deque.
    ///
    /// # Intuition
    /// Use a non-shrinking sliding window: expand right, and if cost exceeds budget,
    /// shrink by exactly one from the left. The window size never decreases, so the
    /// final window size is the answer.
    ///
    /// # Approach
    /// 1. Expand window by adding elements to the right
    /// 2. Maintain monotonic decreasing deque for maximum charge_time
    /// 3. If cost exceeds budget, shrink window by exactly 1 from left
    /// 4. Answer is `n - left` at the end
    ///
    /// # Complexity
    /// - Time: O(n) — each element enters/leaves deque once
    /// - Space: O(n) — for monotonic deque
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = running_costs.len();
        let mut sum: i64 = 0;
        let mut left = 0usize;
        let mut deque: VecDeque<usize> = VecDeque::new();

        for right in 0..n {
            sum += running_costs[right] as i64;

            while deque
                .back()
                .is_some_and(|&back| charge_times[back] <= charge_times[right])
            {
                deque.pop_back();
            }
            deque.push_back(right);

            let max_charge = charge_times[*deque.front().unwrap()] as i64;
            let k = (right - left + 1) as i64;
            if max_charge + k * sum > budget {
                if deque.front() == Some(&left) {
                    deque.pop_front();
                }
                sum -= running_costs[left] as i64;
                left += 1;
            }
        }

        (n - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25),
            3
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19),
            0
        );
    }

    #[test]
    fn test_single_robot_within_budget() {
        assert_eq!(Solution::maximum_robots(vec![5], vec![3], 10), 1);
    }

    #[test]
    fn test_single_robot_exceeds_budget() {
        assert_eq!(Solution::maximum_robots(vec![5], vec![6], 10), 0);
    }

    #[test]
    fn test_all_robots_within_budget() {
        assert_eq!(
            Solution::maximum_robots(vec![1, 2, 3], vec![1, 1, 1], 100),
            3
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(Solution::maximum_robots(vec![], vec![], 100), 0);
    }

    #[test]
    fn test_large_budget() {
        assert_eq!(
            Solution::maximum_robots(
                vec![100000, 100000, 100000],
                vec![100000, 100000, 100000],
                1_000_000_000_000_i64
            ),
            3
        );
    }
}
