impl Solution {
    /// Finds the minimum cost path from any first-row cell to any last-row cell.
    ///
    /// # Intuition
    /// Dynamic programming row by row: for each cell in the current row, consider
    /// all cells in the previous row and pick the minimum (previous cost + move cost
    /// + current cell value).
    ///
    /// # Approach
    /// 1. Initialize DP with the first row's values
    /// 2. For each subsequent row, compute the optimal cost to reach each cell
    /// 3. Return the minimum value in the final DP row
    ///
    /// # Complexity
    /// - Time: O(m × n²) where m = rows, n = columns
    /// - Space: O(n) for the DP array
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = grid[0].clone();

        for i in 1..m {
            let new_dp: Vec<i32> = (0..n)
                .map(|j| {
                    (0..n)
                        .map(|k| dp[k] + move_cost[grid[i - 1][k] as usize][j] + grid[i][j])
                        .min()
                        .unwrap_or(i32::MAX)
                })
                .collect();
            dp = new_dp;
        }

        dp.into_iter().min().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let grid = vec![vec![5, 3], vec![4, 0], vec![2, 1]];
        let move_cost = vec![
            vec![9, 8],
            vec![1, 5],
            vec![10, 12],
            vec![18, 6],
            vec![2, 4],
            vec![14, 3],
        ];
        assert_eq!(Solution::min_path_cost(grid, move_cost), 17);
    }

    #[test]
    fn test_example_two() {
        let grid = vec![vec![5, 1, 2], vec![4, 0, 3]];
        let move_cost = vec![
            vec![12, 10, 15],
            vec![20, 23, 8],
            vec![21, 7, 1],
            vec![8, 1, 13],
            vec![9, 10, 25],
            vec![5, 3, 2],
        ];
        assert_eq!(Solution::min_path_cost(grid, move_cost), 6);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![1, 2, 3]];
        let move_cost: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::min_path_cost(grid, move_cost), 1);
    }
}
