impl Solution {
    /// Finds the maximum profit from a triplet with strictly increasing prices.
    ///
    /// # Intuition
    /// For each middle element `j`, find the best left profit where `price[i] < price[j]`
    /// and the best right profit where `price[k] > price[j]`. The answer is the maximum
    /// sum across all valid middle positions.
    ///
    /// # Approach
    /// 1. For each index `j`, scan left for the maximum profit with a smaller price.
    /// 2. Scan right for the maximum profit with a larger price.
    /// 3. If both sides yield positive profits, update the answer.
    ///
    /// # Complexity
    /// - Time: O(n²) where n is the number of elements
    /// - Space: O(1)
    pub fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut ans = -1;

        for j in 0..n {
            let left = (0..j)
                .filter(|&i| prices[i] < prices[j])
                .map(|i| profits[i])
                .max()
                .unwrap_or(0);

            let right = (j + 1..n)
                .filter(|&k| prices[j] < prices[k])
                .map(|k| profits[k])
                .max()
                .unwrap_or(0);

            if left > 0 && right > 0 {
                ans = ans.max(left + profits[j] + right);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profitable_triplet() {
        assert_eq!(
            Solution::max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]),
            19
        );
    }

    #[test]
    fn test_no_valid_triplet() {
        assert_eq!(Solution::max_profit(vec![3, 2, 1], vec![1, 2, 3]), -1);
    }
}
