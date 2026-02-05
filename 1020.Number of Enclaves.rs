impl Solution {
    /// Counts land cells not reachable from the boundary.
    ///
    /// # Intuition
    /// DFS from all boundary land cells to mark them as water. The remaining
    /// land cells are enclaves.
    ///
    /// # Approach
    /// Iterate over boundary cells; DFS flood-fill each boundary-connected
    /// land cell to 0. Count remaining 1s.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n) recursion stack
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) {
            grid[r][c] = 0;
            let (rows, cols) = (grid.len(), grid[0].len());
            for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr < rows && nc < cols && grid[nr][nc] == 1 {
                    dfs(grid, nr, nc);
                }
            }
        }

        for j in 0..n {
            if grid[0][j] == 1 {
                dfs(&mut grid, 0, j);
            }
            if grid[m - 1][j] == 1 {
                dfs(&mut grid, m - 1, j);
            }
        }
        for i in 0..m {
            if grid[i][0] == 1 {
                dfs(&mut grid, i, 0);
            }
            if grid[i][n - 1] == 1 {
                dfs(&mut grid, i, n - 1);
            }
        }

        grid.iter().flatten().filter(|x| **x == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_enclaves() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 3);
    }

    #[test]
    fn test_no_enclaves() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 0);
    }
}
