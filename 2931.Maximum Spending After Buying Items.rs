impl Solution {
    /// Maximizes total spending by buying cheaper items earlier.
    ///
    /// # Intuition
    /// Day multipliers increase, so any inversion where a larger value is bought earlier than a
    /// smaller one reduces the total. Swapping them increases the sum by the value difference.
    ///
    /// # Approach
    /// Each shop can only be bought from right to left, which yields a nondecreasing sequence of
    /// values because rows are sorted non-increasing. Any interleaving of these sequences is
    /// feasible, and a global nondecreasing order is achievable via a k-way merge. Therefore, we
    /// can flatten all values, sort ascending, and multiply by days `1..=m*n`.
    ///
    /// # Complexity
    /// - Time: O(m · n · log(m · n))
    /// - Space: O(m · n)
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut all_values: Vec<i64> = values
            .into_iter()
            .flat_map(|row| row.into_iter().map(i64::from))
            .collect();
        all_values.sort_unstable();

        all_values
            .iter()
            .enumerate()
            .map(|(day_index, value)| (*value) * (day_index as i64 + 1))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let values = vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]];
        assert_eq!(Solution::max_spending(values), 285);
    }

    #[test]
    fn test_example_2() {
        let values = vec![vec![10, 8, 6, 4, 2], vec![9, 7, 5, 3, 2]];
        assert_eq!(Solution::max_spending(values), 386);
    }

    #[test]
    fn test_single_shop() {
        let values = vec![vec![7, 5, 1]];
        assert_eq!(Solution::max_spending(values), 32);
    }

    #[test]
    fn test_all_equal() {
        let values = vec![vec![3, 3], vec![3, 3]];
        assert_eq!(Solution::max_spending(values), 30);
    }
}
