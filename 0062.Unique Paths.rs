impl Solution {
    /// Space-optimized dynamic programming for grid unique paths.
    ///
    /// # Intuition
    /// The number of paths to any cell is the sum of paths from the cell
    /// above and the cell to the left. Since each row only depends on the
    /// previous row, a single 1D array suffices.
    ///
    /// # Approach
    /// Initialize a row of ones (only one way to reach any cell in the
    /// first row). For each subsequent row, accumulate left-to-right:
    /// `f[j] += f[j-1]`, where `f[j]` already holds the value from the
    /// row above.
    ///
    /// # Complexity
    /// - Time: O(m × n) — filling the DP array
    /// - Space: O(n) — single row array
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];

        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }

        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_seven() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn three_by_two() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn one_by_one() {
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::unique_paths(1, 5), 1);
    }

    #[test]
    fn single_column() {
        assert_eq!(Solution::unique_paths(5, 1), 1);
    }
}
