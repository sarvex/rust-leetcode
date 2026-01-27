impl Solution {
    /// Maximizes profit with at most two stock transactions using state machine DP.
    ///
    /// # Intuition
    /// Track four states: holding after first buy, sold after first sell,
    /// holding after second buy, sold after second sell. Each state transitions
    /// greedily at every price.
    ///
    /// # Approach
    /// 1. Initialize four states representing the two buy-sell cycles.
    /// 2. For each price, update states to reflect the best decision at that point.
    /// 3. The answer is the final state after the second sell.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy1 = -prices[0];
        let mut sell1 = 0;
        let mut buy2 = -prices[0];
        let mut sell2 = 0;

        for &price in prices.iter().skip(1) {
            buy1 = buy1.max(-price);
            sell1 = sell1.max(buy1 + price);
            buy2 = buy2.max(sell1 - price);
            sell2 = sell2.max(buy2 + price);
        }

        sell2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_transactions() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn one_transaction_optimal() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn no_profit() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_price() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
