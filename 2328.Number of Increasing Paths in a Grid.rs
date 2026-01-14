impl Solution {
    /// Count all strictly increasing paths in a grid with 4-directional movement.
    ///
    /// # Intuition
    /// Each cell can be the start of multiple increasing paths. A path starting at cell (i,j)
    /// can extend to any adjacent cell with a strictly greater value. By processing cells
    /// in descending order of their values, we ensure all valid destinations are computed
    /// before processing any source cell.
    ///
    /// # Approach
    /// 1. Collect all cells as (value, row, col) tuples
    /// 2. Sort cells by value in descending order (topological ordering by value)
    /// 3. For each cell, compute dp[i][j] = 1 + sum of dp values for neighbors with higher values
    /// 4. Since processing in descending order, all higher-value neighbors are already computed
    /// 5. Accumulate total paths across all cells
    ///
    /// # Complexity
    /// - Time: O(m * n * log(m * n)) for sorting, O(m * n) for DP traversal
    /// - Space: O(m * n) for dp array and sorted cells vector
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let rows = grid.len();
        let cols = grid[0].len();

        // Collect all cells with their values for sorting
        let mut cells: Vec<(i32, usize, usize)> = Vec::with_capacity(rows * cols);
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                cells.push((val, i, j));
            }
        }

        // Sort by value descending - enables bottom-up DP without recursion
        cells.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        // dp[i][j] = number of increasing paths starting from cell (i, j)
        let mut dp = vec![vec![0i64; cols]; rows];
        let mut total_paths: i64 = 0;

        for (val, row, col) in cells {
            // Each cell is at minimum a path of length 1
            let mut path_count: i64 = 1;

            // Add paths from neighbors with strictly greater values
            for (dr, dc) in DIRECTIONS {
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if grid[nr][nc] > val {
                        path_count = (path_count + dp[nr][nc]) % MOD;
                    }
                }
            }

            dp[row][col] = path_count;
            total_paths = (total_paths + path_count) % MOD;
        }

        total_paths as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![1, 1], vec![3, 4]];
        assert_eq!(Solution::count_paths(grid), 8);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![1], vec![2]];
        assert_eq!(Solution::count_paths(grid), 3);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(Solution::count_paths(grid), 1);
    }

    #[test]
    fn test_all_same_values() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        // No increasing paths possible, only 6 single-cell paths
        assert_eq!(Solution::count_paths(grid), 6);
    }

    #[test]
    fn test_strictly_increasing_row() {
        let grid = vec![vec![1, 2, 3]];
        // Paths: [1], [2], [3], [1->2], [2->3], [1->2->3] = 6
        assert_eq!(Solution::count_paths(grid), 6);
    }

    #[test]
    fn test_strictly_increasing_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        // Same as row case: 6 paths
        assert_eq!(Solution::count_paths(grid), 6);
    }

    #[test]
    fn test_larger_grid() {
        let grid = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        // Multiple increasing paths available
        let result = Solution::count_paths(grid);
        assert!(result > 0);
    }
}
