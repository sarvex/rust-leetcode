impl Solution {
    /// Calculates minimum initial money required to complete all transactions in any order.
    ///
    /// # Intuition
    /// To survive the worst-case ordering, we need enough money to cover all net losses
    /// plus the upfront cost of whichever transaction we perform last.
    ///
    /// # Approach
    /// 1. Sum all net losses: transactions where cost exceeds cashback contribute (cost - cashback)
    /// 2. For the "last" transaction in worst-case ordering:
    ///    - If losing (cost > cashback): we need total_loss + cashback (since we already counted cost - cashback)
    ///    - If winning (cost <= cashback): we need total_loss + cost (no loss was counted for this transaction)
    /// 3. Track maximum required money across all possible "last" transactions
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through transactions
    /// - Space: O(1) — constant extra space
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let (total_loss, max_additional) =
            transactions
                .iter()
                .fold((0i64, 0i32), |(loss_sum, max_add), transaction| {
                    let (cost, cashback) = (transaction[0], transaction[1]);
                    let net_loss = (cost - cashback).max(0);
                    let additional = if cost > cashback { cashback } else { cost };

                    (loss_sum + net_loss as i64, max_add.max(additional))
                });

        total_loss + max_additional as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let transactions = vec![vec![2, 1], vec![5, 0], vec![4, 2]];
        assert_eq!(Solution::minimum_money(transactions), 10);
    }

    #[test]
    fn test_example_2() {
        let transactions = vec![vec![3, 0], vec![0, 3]];
        assert_eq!(Solution::minimum_money(transactions), 3);
    }

    #[test]
    fn test_single_winning_transaction() {
        let transactions = vec![vec![1, 5]];
        assert_eq!(Solution::minimum_money(transactions), 1);
    }

    #[test]
    fn test_single_losing_transaction() {
        let transactions = vec![vec![5, 1]];
        assert_eq!(Solution::minimum_money(transactions), 5);
    }

    #[test]
    fn test_all_winning_transactions() {
        let transactions = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::minimum_money(transactions), 3);
    }

    #[test]
    fn test_all_losing_transactions() {
        let transactions = vec![vec![4, 1], vec![5, 2], vec![6, 3]];
        assert_eq!(Solution::minimum_money(transactions), 12);
    }

    #[test]
    fn test_break_even_transactions() {
        let transactions = vec![vec![3, 3], vec![5, 5]];
        assert_eq!(Solution::minimum_money(transactions), 5);
    }
}
