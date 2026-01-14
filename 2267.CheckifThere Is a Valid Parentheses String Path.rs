impl Solution {
    /// Determines if there exists a path from top-left to bottom-right forming valid parentheses.
    ///
    /// # Intuition
    /// A valid parentheses string requires equal '(' and ')' with the count of '(' always >= ')'
    /// at any prefix. We track "balance" (open minus close) as we traverse, ensuring it never
    /// goes negative and equals zero at the destination.
    ///
    /// # Approach
    /// Use DFS with memoization where state is (row, col, balance). At each cell, we update
    /// balance (+1 for '(', -1 for ')') and explore right/down moves. Prune paths where
    /// balance goes negative or exceeds remaining cells.
    ///
    /// # Complexity
    /// - Time: O(m * n * (m + n)) - each state visited once
    /// - Space: O(m * n * (m + n)) - memoization table size
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // Path length is m + n - 1, must be even for valid parentheses
        if (m + n) % 2 == 0 {
            return false;
        }

        // Must start with '(' and end with ')'
        if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' {
            return false;
        }

        // Maximum possible balance is half the path length
        let max_balance = (m + n) / 2 + 1;
        let mut visited = vec![vec![vec![false; max_balance + 1]; n]; m];

        Self::dfs(&grid, 0, 0, 0, &mut visited)
    }

    fn dfs(
        grid: &[Vec<char>],
        row: usize,
        col: usize,
        balance: i32,
        visited: &mut [Vec<Vec<bool>>],
    ) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // Update balance based on current cell
        let new_balance = if grid[row][col] == '(' {
            balance + 1
        } else {
            balance - 1
        };

        // Prune: balance went negative
        if new_balance < 0 {
            return false;
        }

        // Prune: balance exceeds what remaining path can close
        let remaining = (m - 1 - row) + (n - 1 - col);
        if new_balance as usize > remaining {
            return false;
        }

        // Reached destination
        if row == m - 1 && col == n - 1 {
            return new_balance == 0;
        }

        let balance_idx = new_balance as usize;

        // Check memoization
        if visited[row][col][balance_idx] {
            return false;
        }
        visited[row][col][balance_idx] = true;

        // Explore down
        if row + 1 < m && Self::dfs(grid, row + 1, col, new_balance, visited) {
            return true;
        }

        // Explore right
        if col + 1 < n && Self::dfs(grid, row, col + 1, new_balance, visited) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path_exists() {
        // Example 1: 4x3 grid, path length = 6 (even)
        let grid = vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')'],
        ];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_no_valid_path() {
        // Example 2: 2x2 grid, path length = 3 (odd) -> always false
        let grid = vec![vec![')', ')'], vec!['(', '(']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_single_cell_open() {
        let grid = vec![vec!['(']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_single_cell_close() {
        let grid = vec![vec![')']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_simple_valid_horizontal() {
        // 1x2 grid, path length = 2 (even)
        let grid = vec![vec!['(', ')']];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_simple_valid_vertical() {
        // 2x1 grid, path length = 2 (even)
        let grid = vec![vec!['('], vec![')']];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_3x2_nested_valid() {
        // 3x2 grid, path length = 4 (even)
        let grid = vec![vec!['(', '('], vec!['(', ')'], vec![')', ')']];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_2x3_valid_path() {
        // 2x3 grid, path length = 4 (even)
        let grid = vec![vec!['(', '(', ')'], vec!['(', ')', ')']];
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_starts_with_close() {
        // 2x3 grid starting with ')' -> invalid
        let grid = vec![vec![')', '(', ')'], vec!['(', ')', ')']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_ends_with_open() {
        // 2x3 grid ending with '(' -> invalid
        let grid = vec![vec!['(', '(', '('], vec![')', ')', '(']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_odd_path_length() {
        // 2x2 grid, path length = 3 (odd) -> always false
        let grid = vec![vec!['(', '('], vec![')', ')']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_balance_goes_negative() {
        // 2x3 grid where all paths go negative at some point
        let grid = vec![vec!['(', ')', ')'], vec![')', ')', ')']];
        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn test_large_valid() {
        // 3x4 grid, path length = 6 (even)
        let grid = vec![
            vec!['(', '(', '(', ')'],
            vec![')', ')', '(', ')'],
            vec!['(', '(', ')', ')'],
        ];
        assert!(Solution::has_valid_path(grid));
    }
}
