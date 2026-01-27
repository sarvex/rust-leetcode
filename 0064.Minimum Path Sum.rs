impl Solution {
    /// In-place dynamic programming for minimum path sum in a grid.
    ///
    /// # Intuition
    /// Each cell's minimum path cost is its own value plus the minimum of
    /// the cost from above or from the left. Processing row-by-row after
    /// initializing edges allows reuse of the input grid.
    ///
    /// # Approach
    /// Accumulate the first column (each cell adds the cell above) and the
    /// first row (each cell adds the cell to the left). For interior cells,
    /// add the minimum of the top and left neighbors. The bottom-right cell
    /// holds the answer.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each cell processed once
    /// - Space: O(1) — modifies the input grid in-place
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
        }
        for j in 1..n {
            grid[0][j] += grid[0][j - 1];
        }
        for i in 1..m {
            for j in 1..n {
                grid[i][j] += grid[i - 1][j].min(grid[i][j - 1]);
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn two_by_three() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::min_path_sum(grid), 12);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::min_path_sum(vec![vec![5]]), 5);
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3]]), 6);
    }
}
