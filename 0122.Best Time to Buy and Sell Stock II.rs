impl Solution {
    /// Maximizes profit from unlimited stock transactions using greedy summation of positive diffs.
    ///
    /// # Intuition
    /// With unlimited transactions, every positive price increase can be captured.
    /// Summing all consecutive gains yields the maximum profit.
    ///
    /// # Approach
    /// 1. Iterate through consecutive price pairs.
    /// 2. Add the difference whenever today's price exceeds yesterday's.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_transactions() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn ascending_prices() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn descending_prices() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_price() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }
}
