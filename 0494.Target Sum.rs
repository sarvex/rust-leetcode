impl Solution {
    /// Counts ways to assign +/- signs to reach the target using subset sum DP.
    ///
    /// # Intuition
    /// Partitioning into positive set P and negative set N yields
    /// P - N = target and P + N = sum, so P = (sum + target) / 2. This
    /// reduces to counting subsets summing to a specific value.
    ///
    /// # Approach
    /// 1. Compute sum; check feasibility (sum >= target, even difference).
    /// 2. Use 1-D DP for the 0/1 knapsack counting variant with capacity
    ///    (sum - target) / 2.
    ///
    /// # Complexity
    /// - Time: O(n × capacity)
    /// - Space: O(capacity)
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum < target.abs() || (sum - target) % 2 != 0 {
            return 0;
        }
        let capacity = ((sum - target) / 2) as usize;
        let mut dp = vec![0i32; capacity + 1];
        dp[0] = 1;
        for &num in &nums {
            let v = num as usize;
            for j in (v..=capacity).rev() {
                dp[j] += dp[j - v];
            }
        }
        dp[capacity]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn test_impossible() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 2), 0);
    }
}
