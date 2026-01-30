
impl Solution {
    /// Minimizes the last stone weight using 0/1 knapsack DP.
    ///
    /// # Intuition
    /// Partition stones into two groups minimizing the difference of their
    /// sums. This is equivalent to a 0/1 knapsack targeting sum/2.
    ///
    /// # Approach
    /// Compute total sum. DP with capacity `sum/2`: for each stone, update
    /// the maximum achievable sum. Answer is `total - 2 * dp[sum/2]`.
    ///
    /// # Complexity
    /// - Time: O(n * sum)
    /// - Space: O(sum) with 1D DP optimization
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let total: i32 = stones.iter().sum();
        let half = (total / 2) as usize;
        let mut dp = vec![0; half + 1];

        for &stone in &stones {
            let s = stone as usize;
            for j in (s..=half).rev() {
                dp[j] = dp[j].max(dp[j - s] + stone);
            }
        }

        total - 2 * dp[half]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test_equal_split() {
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::last_stone_weight_ii(vec![1]), 1);
    }
}
