impl Solution {
    /// Top-down memoized DFS for unique paths with obstacles.
    ///
    /// # Intuition
    /// Same structure as the obstacle-free version, but cells with obstacles
    /// contribute zero paths. Memoization prevents recomputation of
    /// overlapping subproblems.
    ///
    /// # Approach
    /// Use DFS from `(0, 0)` to `(m-1, n-1)`. Return 0 for out-of-bounds
    /// or obstacle cells, 1 for the destination. Cache results in a 2D
    /// memo table initialized to -1. At each cell, the answer is the sum
    /// of paths going right and down.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each cell computed at most once
    /// - Space: O(m × n) — memo table and recursion stack
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut memo = vec![vec![-1i32; n]; m];

        fn dfs(grid: &[Vec<i32>], memo: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if i >= m || j >= n || grid[i][j] == 1 {
                return 0;
            }
            if i == m - 1 && j == n - 1 {
                return 1;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            memo[i][j] = dfs(grid, memo, i + 1, j) + dfs(grid, memo, i, j + 1);
            memo[i][j]
        }

        dfs(&obstacle_grid, &mut memo, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obstacle_in_middle() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }

    #[test]
    fn obstacle_at_start() {
        let grid = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 0);
    }

    #[test]
    fn no_obstacles() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }

    #[test]
    fn single_cell_no_obstacle() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
    }

    #[test]
    fn single_cell_with_obstacle() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
    }
}
