const INF: i64 = 1_i64 << 60;

fn compute_dp_for_target(nums: &[i32], target: i32, prev_dp: Option<&[i64]>) -> Vec<i64> {
    let mut curr_dp = vec![INF; nums.len()];
    let mut prev_list: Vec<(i32, i64)> = Vec::new();

    for (idx, &num) in nums.iter().enumerate() {
        let mut new_list = Vec::with_capacity(prev_list.len() + 1);
        let start_cost = match prev_dp {
            None => {
                if idx == 0 {
                    0
                } else {
                    INF
                }
            }
            Some(prev) => {
                if idx == 0 {
                    INF
                } else {
                    prev[idx - 1]
                }
            }
        };

        if start_cost < INF {
            new_list.push((num, start_cost));
        }

        for &(and_val, cost) in &prev_list {
            let new_and = and_val & num;
            if let Some(last) = new_list.last_mut() {
                if last.0 == new_and {
                    if cost < last.1 {
                        last.1 = cost;
                    }
                } else {
                    new_list.push((new_and, cost));
                }
            } else {
                new_list.push((new_and, cost));
            }
        }

        let mut best_prev = INF;
        for &(and_val, cost) in &new_list {
            if and_val == target && cost < best_prev {
                best_prev = cost;
            }
        }

        if best_prev < INF {
            curr_dp[idx] = best_prev + num as i64;
        }

        prev_list = new_list;
    }

    curr_dp
}

impl Solution {
    /// Dynamic programming with compressed subarray AND states.
    ///
    /// # Intuition
    /// Extending a subarray can only decrease its bitwise AND, so each end index
    /// has only a small number of distinct AND values to consider.
    ///
    /// # Approach
    /// - For each target AND value, scan nums left to right.
    /// - Track all distinct AND results for subarrays ending at the current
    ///   position, keeping the smallest previous-partition cost for each.
    /// - When the target AND appears, update the dp cost by adding nums[i] as
    ///   the segment's value.
    ///
    /// # Complexity
    /// - Time: O(m * n * B), B <= 32 distinct AND values per end.
    /// - Space: O(n + B)
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        if and_values.len() > nums.len() || nums.is_empty() {
            return -1;
        }

        let mut prev_dp: Option<Vec<i64>> = None;
        for &target in &and_values {
            let next_dp = compute_dp_for_target(&nums, target, prev_dp.as_deref());
            prev_dp = Some(next_dp);
        }

        let answer = prev_dp
            .and_then(|dp| dp.last().copied())
            .unwrap_or(INF);

        if answer >= INF {
            -1
        } else {
            answer as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 4, 3, 3, 2];
        let and_values = vec![0, 3, 3, 2];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 12);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 3, 5, 7, 7, 7, 5];
        let and_values = vec![0, 7, 5];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 17);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3, 4];
        let and_values = vec![2];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), -1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![7];
        let and_values = vec![7];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 7);
    }

    #[test]
    fn test_all_zeroes() {
        let nums = vec![0, 0, 0];
        let and_values = vec![0, 0, 0];
        assert_eq!(Solution::minimum_value_sum(nums, and_values), 0);
    }
}
