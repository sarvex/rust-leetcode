impl Solution {
    /// Deletes the greatest value from each row repeatedly and sums the maxima.
    ///
    /// # Intuition
    /// Sorting each row lets us process columns right-to-left, where each column
    /// contributes the maximum of its values across all rows.
    ///
    /// # Approach
    /// 1. Sort each row in ascending order
    /// 2. For each column index, take the maximum across all rows and accumulate
    ///
    /// # Complexity
    /// - Time: O(m × n log n) — sorting each row
    /// - Space: O(1) — in-place sort
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        grid.iter_mut().for_each(|row| row.sort_unstable());

        (0..grid[0].len())
            .map(|j| grid.iter().map(|row| row[j]).max().unwrap_or(0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = vec![vec![1, 2, 4], vec![3, 3, 1]];
        assert_eq!(Solution::delete_greatest_value(grid), 8);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![10]];
        assert_eq!(Solution::delete_greatest_value(grid), 10);
    }

    #[test]
    fn test_uniform_grid() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::delete_greatest_value(grid), 2);
    }
}
