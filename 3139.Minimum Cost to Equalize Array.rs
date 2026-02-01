const MOD: i64 = 1_000_000_007;

fn ceil_div(value: i64, divisor: i64) -> i64 {
    if value >= 0 {
        (value + divisor - 1) / divisor
    } else {
        value / divisor
    }
}

fn cost_for_target(target: i64, n: i64, sum: i64, min_value: i64, cost1: i64, cost2: i64) -> i128 {
    let total_increments = n * target - sum;
    if total_increments == 0 {
        return 0;
    }
    let max_deficit = target - min_value;
    let pair_ops = (total_increments / 2).min(total_increments - max_deficit);
    let single_ops = total_increments - 2 * pair_ops;
    pair_ops as i128 * cost2 as i128 + single_ops as i128 * cost1 as i128
}

impl Solution {
    /// Greedy pairing with a boundary search on the final target.
    ///
    /// # Intuition
    /// Pairing increments is cheaper when `cost2 < 2 * cost1`, so we want to maximize the number of
    /// paired operations. The maximum pairing depends on the total deficit and the largest deficit.
    ///
    /// # Approach
    /// For a target value `T`, let `S` be the total deficit and `D` the largest deficit. The maximum
    /// number of pair operations is `min(S / 2, S - D)`. When `cost2 >= 2 * cost1`, single operations
    /// dominate, and the best target is the current maximum. Otherwise, the cost is piecewise linear
    /// with one boundary where `D <= S / 2`, so it suffices to evaluate a small set of candidate
    /// targets around that boundary and the current maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let n = nums.len() as i64;
        if n <= 1 {
            return 0;
        }

        let mut min_value = i64::MAX;
        let mut max_value = i64::MIN;
        let mut sum = 0i64;
        for &num in &nums {
            let value = num as i64;
            min_value = min_value.min(value);
            max_value = max_value.max(value);
            sum += value;
        }

        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;
        let total_at_max = n * max_value - sum;

        if cost2 >= 2 * cost1 {
            let cost = total_at_max as i128 * cost1 as i128;
            return (cost % MOD as i128) as i32;
        }

        if n == 2 {
            let cost = total_at_max as i128 * cost1 as i128;
            return (cost % MOD as i128) as i32;
        }

        let mut candidates = Vec::with_capacity(5);
        candidates.push(max_value);
        candidates.push(max_value + 1);

        let threshold = ceil_div(sum - 2 * min_value, n - 2);
        let base = max_value.max(threshold);
        candidates.push(base);
        candidates.push(base + 1);
        if base > max_value {
            candidates.push(base - 1);
        }

        candidates.sort_unstable();
        candidates.dedup();

        let mut best = i128::MAX;
        for target in candidates {
            if target < max_value {
                continue;
            }
            let cost = cost_for_target(target, n, sum, min_value, cost1, cost2);
            if cost < best {
                best = cost;
            }
        }

        (best % MOD as i128) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![4, 1];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 5, 2), 15);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 3, 3, 3, 5];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 2, 1), 6);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 5, 3];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 1, 3), 4);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![7];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 3, 1), 0);
    }

    #[test]
    fn test_two_elements_prefers_current_max() {
        let nums = vec![1, 10];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 5, 1), 45);
    }

    #[test]
    fn test_cost2_ge_double_cost1() {
        let nums = vec![1, 3, 5];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 2, 5), 12);
    }

    #[test]
    fn test_raise_target_when_pairs_are_cheaper() {
        let nums = vec![1, 100, 100];
        assert_eq!(Solution::min_cost_to_equalize_array(nums, 10, 1), 198);
    }
}
