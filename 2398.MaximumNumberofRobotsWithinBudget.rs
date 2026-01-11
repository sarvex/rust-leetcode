use std::collections::VecDeque;

impl Solution {
    /// Maximum Number of Robots Within Budget
    ///
    /// # Intuition
    /// Use sliding window where the window size is monotonically non-decreasing.
    /// Instead of shrinking until valid, shrink by at most 1 per iteration.
    /// The maximum valid window size is simply `n - left` at the end.
    ///
    /// # Approach
    /// 1. Expand window by adding elements to the right
    /// 2. Maintain monotonic decreasing deque for maximum charge_time
    /// 3. If cost exceeds budget, shrink window by exactly 1 from left
    /// 4. Window size never decreases, final answer is `n - left`
    ///
    /// # Complexity
    /// - Time: O(n) - single pass, each element enters/leaves deque once
    /// - Space: O(n) - for monotonic deque
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = running_costs.len();
        let mut sum: i64 = 0;
        let mut left = 0usize;
        let mut deque = VecDeque::new();

        for right in 0..n {
            sum += running_costs[right] as i64;

            // Maintain monotonic decreasing deque for maximum
            while !deque.is_empty() && charge_times[*deque.back().unwrap()] <= charge_times[right] {
                deque.pop_back();
            }
            deque.push_back(right);

            // If cost exceeds budget, shrink window by 1
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

    struct Solution;

    #[test]
    fn test_example_1() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_example_2() {
        let charge_times = vec![11, 12, 19];
        let running_costs = vec![10, 8, 7];
        let budget = 19;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            0
        );
    }

    #[test]
    fn test_single_robot_within_budget() {
        let charge_times = vec![5];
        let running_costs = vec![3];
        let budget = 10;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            1
        );
    }

    #[test]
    fn test_single_robot_exceeds_budget() {
        let charge_times = vec![5];
        let running_costs = vec![6];
        let budget = 10;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            0
        );
    }

    #[test]
    fn test_all_robots_within_budget() {
        let charge_times = vec![1, 2, 3];
        let running_costs = vec![1, 1, 1];
        let budget = 100;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_large_budget() {
        let charge_times = vec![100000, 100000, 100000];
        let running_costs = vec![100000, 100000, 100000];
        let budget = 1_000_000_000_000_i64;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_decreasing_charge_times() {
        let charge_times = vec![10, 8, 6, 4, 2];
        let running_costs = vec![1, 1, 1, 1, 1];
        let budget = 20;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_increasing_charge_times() {
        let charge_times = vec![2, 4, 6, 8, 10];
        let running_costs = vec![1, 1, 1, 1, 1];
        let budget = 20;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_empty_input() {
        let charge_times: Vec<i32> = vec![];
        let running_costs: Vec<i32> = vec![];
        let budget = 100;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            0
        );
    }

    #[test]
    fn test_tight_budget() {
        let charge_times = vec![1, 2, 3, 4, 5];
        let running_costs = vec![1, 2, 3, 4, 5];
        let budget = 7;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            2
        );
    }
}
