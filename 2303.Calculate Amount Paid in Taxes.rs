impl Solution {
    /// Calculates total tax paid using progressive tax brackets.
    ///
    /// # Intuition
    /// Each bracket taxes only the income within its range. We track the previous
    /// bracket boundary to compute the taxable amount per tier.
    ///
    /// # Approach
    /// Fold over brackets, accumulating tax by computing `(min(income, upper) - prev) * rate`
    /// for each tier, stopping early when income is fully taxed.
    ///
    /// # Complexity
    /// - Time: O(b) where b = number of brackets
    /// - Space: O(1)
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        brackets
            .iter()
            .fold((0.0f64, 0i32), |(tax, prev), bracket| {
                let upper = bracket[0];
                let rate = bracket[1];
                let taxable = income.min(upper) - prev;
                (tax + f64::from(taxable) * f64::from(rate) * 0.01, upper)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let brackets = vec![vec![3, 50], vec![7, 10], vec![12, 25]];
        let result = Solution::calculate_tax(brackets, 10);
        assert!((result - 2.65).abs() < 1e-5);
    }

    #[test]
    fn test_example_two() {
        let brackets = vec![vec![1, 0], vec![4, 25], vec![5, 50]];
        let result = Solution::calculate_tax(brackets, 2);
        assert!((result - 0.25).abs() < 1e-5);
    }

    #[test]
    fn test_zero_income() {
        let brackets = vec![vec![10, 50]];
        let result = Solution::calculate_tax(brackets, 0);
        assert!((result - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_income_in_first_bracket() {
        let brackets = vec![vec![5, 10], vec![10, 20]];
        let result = Solution::calculate_tax(brackets, 3);
        assert!((result - 0.3).abs() < 1e-5);
    }
}
