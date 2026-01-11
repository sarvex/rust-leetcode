/// Maximum Number of Robots Within Budget
///
/// # Intuition
/// We need to find the longest consecutive sequence of robots whose total cost
/// doesn't exceed the budget. The cost formula involves both max(chargeTimes)
/// and sum(runningCosts), suggesting a sliding window approach with efficient
/// maximum tracking.
///
/// # Approach
/// Use a sliding window with a monotonic deque to efficiently track the maximum
/// charge time within the current window:
/// 1. Maintain a decreasing monotonic deque storing indices of charge times
/// 2. The front of the deque always contains the index of the maximum element
/// 3. Expand the window by adding elements from the right
/// 4. Shrink the window from the left when cost exceeds budget
/// 5. Track the maximum valid window size
///
/// # Complexity
/// - Time: O(n) - each element is added and removed from deque at most once
/// - Space: O(n) - for the monotonic deque in worst case
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();
        let mut max_robots = 0;
        let mut running_sum: i64 = 0;
        let mut left = 0;

        // Monotonic deque storing indices, maintains decreasing order of charge_times
        let mut deque: VecDeque<usize> = VecDeque::new();

        for right in 0..n {
            // Add current running cost to sum
            running_sum += running_costs[right] as i64;

            // Maintain monotonic decreasing deque for charge_times
            while let Some(&back_idx) = deque.back() {
                if charge_times[back_idx] <= charge_times[right] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(right);

            // Calculate current window cost
            // cost = max(charge_times) + k * sum(running_costs)
            let window_size = (right - left + 1) as i64;
            let max_charge = charge_times[*deque.front().unwrap()] as i64;
            let mut total_cost = max_charge + window_size * running_sum;

            // Shrink window from left while cost exceeds budget
            while total_cost > budget && left <= right {
                // Remove left element from running sum
                running_sum -= running_costs[left] as i64;

                // Remove left index from deque if it's the front
                if let Some(&front_idx) = deque.front() {
                    if front_idx == left {
                        deque.pop_front();
                    }
                }

                left += 1;

                // Recalculate cost if window is non-empty
                if left <= right {
                    let window_size = (right - left + 1) as i64;
                    let max_charge = charge_times[*deque.front().unwrap()] as i64;
                    total_cost = max_charge + window_size * running_sum;
                }
            }

            // Update maximum robots if window is valid
            if left <= right {
                let current_size = right - left + 1;
                max_robots = max_robots.max(current_size);
            }
        }

        max_robots as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

struct Solution;
