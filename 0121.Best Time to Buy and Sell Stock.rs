impl Solution {
    /// Single-pass minimum tracking for maximum stock profit.
    ///
    /// # Intuition
    /// The maximum profit from a single buy-sell transaction is the largest
    /// difference `price[j] - price[i]` where `j > i`. Tracking the running
    /// minimum buy price lets us compute the best sell profit at each day.
    ///
    /// # Approach
    /// Iterate through prices, maintaining the minimum price seen so far.
    /// At each price, compute the potential profit and update the maximum.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass
    /// - Space: O(1) — two scalar variables
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(min_price, max_profit), &price| {
                let min_price = min_price.min(price);
                (min_price, max_profit.max(price - min_price))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profit_exists() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn no_profit() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn empty_prices() {
        assert_eq!(Solution::max_profit(vec![]), 0);
    }

    #[test]
    fn single_price() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }

    #[test]
    fn ascending() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }
}
