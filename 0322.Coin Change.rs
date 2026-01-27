impl Solution {
    /// Finds the minimum coins to make an amount using unbounded knapsack DP.
    ///
    /// # Intuition
    /// For each coin, update the minimum number of coins needed for every amount.
    /// This is a classic unbounded knapsack where each coin can be used unlimited times.
    ///
    /// # Approach
    /// 1. Initialize dp[0] = 0, all others to amount + 1 (sentinel).
    /// 2. For each coin, update dp[j] = min(dp[j], dp[j - coin] + 1).
    /// 3. If dp[amount] exceeds amount, return -1.
    ///
    /// # Complexity
    /// - Time: O(amount * coins.len())
    /// - Space: O(amount)
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = amount as usize;
        let mut dp = vec![n + 1; n + 1];
        dp[0] = 0;
        for &coin in &coins {
            let c = coin as usize;
            for j in c..=n {
                dp[j] = dp[j].min(dp[j - c] + 1);
            }
        }
        if dp[n] > n {
            -1
        } else {
            dp[n] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn impossible() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn zero_amount() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
