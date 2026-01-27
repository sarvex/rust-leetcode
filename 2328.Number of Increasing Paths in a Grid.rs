impl Solution {
    /// Counts all strictly increasing paths in a grid with 4-directional movement.
    ///
    /// # Intuition
    /// Each cell can be the start of multiple increasing paths. By processing cells
    /// in descending order of value, all valid destinations are computed before any
    /// source cell, enabling bottom-up DP without recursion.
    ///
    /// # Approach
    /// 1. Collect all cells as (value, row, col) tuples
    /// 2. Sort cells by value in descending order (topological ordering by value)
    /// 3. For each cell, dp[i][j] = 1 + sum of dp values for neighbors with higher values
    /// 4. Accumulate total paths across all cells
    ///
    /// # Complexity
    /// - Time: O(m * n * log(m * n)) for sorting, O(m * n) for DP traversal
    /// - Space: O(m * n) for dp array and sorted cells vector
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let (rows, cols) = (grid.len(), grid[0].len());

        let mut cells: Vec<(i32, usize, usize)> = Vec::with_capacity(rows * cols);
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                cells.push((val, i, j));
            }
        }
        cells.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut dp = vec![vec![0i64; cols]; rows];
        let mut total_paths: i64 = 0;

        for (val, row, col) in cells {
            let mut path_count: i64 = 1;

            for (dr, dc) in DIRECTIONS {
                let (nr, nc) = (row as i32 + dr, col as i32 + dc);
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
        assert_eq!(Solution::count_paths(vec![vec![1, 1], vec![3, 4]]), 8);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_paths(vec![vec![1], vec![2]]), 3);
    }

    #[test]
    fn test_single_cell() {
        assert_eq!(Solution::count_paths(vec![vec![5]]), 1);
    }

    #[test]
    fn test_all_same_values() {
        assert_eq!(Solution::count_paths(vec![vec![1, 1, 1], vec![1, 1, 1]]), 6);
    }

    #[test]
    fn test_strictly_increasing_row() {
        assert_eq!(Solution::count_paths(vec![vec![1, 2, 3]]), 6);
    }

    #[test]
    fn test_strictly_increasing_column() {
        assert_eq!(Solution::count_paths(vec![vec![1], vec![2], vec![3]]), 6);
    }
}
