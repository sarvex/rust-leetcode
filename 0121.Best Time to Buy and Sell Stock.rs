/// Finds the maximum profit by buying and selling a stock once.
///
/// # Intuition
/// To maximize profit, we need to buy at the lowest price and sell at the highest price after buying.
/// We use two pointers: one for the buy day and one for the sell day, moving forward in time.
/// When we find a day with a lower price than our current buy day, we update the buy pointer.
///
/// # Approach
/// 1. Initialize two pointers: buy at index 0 and sell at index 1
/// 2. Initialize maximum profit to 0 (if no profit is possible)
/// 3. While the sell pointer is within bounds:
///    a. If price at sell > price at buy, calculate potential profit and update max profit
///    b. Else, update buy pointer to sell (found a better buying opportunity)
///    c. Increment sell pointer to check the next day
/// 4. Return the maximum profit
///
/// # Complexity
/// - Time complexity: O(n), where n is the number of prices
/// - Space complexity: O(1), using only constant extra space
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        
        let mut buy = 0;
        let mut sell = 1;
        let mut max_profit = 0;
        
        while sell < n {
            // If current sell price is higher than buy price, calculate potential profit
            if prices[sell] > prices[buy] {
                let current_profit = prices[sell] - prices[buy];
                max_profit = max_profit.max(current_profit);
            } else {
                // Found a lower buy price, update buy pointer
                buy = sell;
            }
            
            // Move sell pointer to next day
            sell += 1;
        }
        
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
