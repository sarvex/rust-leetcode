impl Solution {
    /// Buy two cheapest chocolates and return remaining money.
    ///
    /// # Intuition
    /// The minimum spend uses the two cheapest items. Sorting reveals them.
    ///
    /// # Approach
    /// 1. Sort prices in ascending order.
    /// 2. Sum the two smallest prices.
    /// 3. Return leftover money if affordable, otherwise return original money.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        prices.sort_unstable();
        let cost = prices[0] + prices[1];
        match cost <= money {
            true => money - cost,
            false => money,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affordable_purchase_returns_leftover() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
    }

    #[test]
    fn too_expensive_returns_original_money() {
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }

    #[test]
    fn exact_money_returns_zero() {
        assert_eq!(Solution::buy_choco(vec![1, 1], 2), 0);
    }
}
