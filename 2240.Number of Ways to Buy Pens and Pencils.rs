impl Solution {
    /// Counts number of distinct ways to buy pens and pencils within budget.
    ///
    /// # Intuition
    /// For each possible count of pens purchased, the remaining budget determines
    /// how many pencils can be bought, giving (remaining / cost2 + 1) choices.
    ///
    /// # Approach
    /// Iterate over all valid pen counts (0..=total/cost1) and accumulate the
    /// number of pencil options for each using fold.
    ///
    /// # Complexity
    /// - Time: O(total / cost1)
    /// - Space: O(1)
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        (0..=total / cost1).fold(0i64, |acc, pen| {
            acc + ((total - pen * cost1) / cost2) as i64 + 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(20, 10, 5), 9);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(5, 10, 10), 1);
    }

    #[test]
    fn test_zero_budget() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(0, 1, 1), 1);
    }

    #[test]
    fn test_equal_costs() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(4, 2, 2), 5);
    }
}
