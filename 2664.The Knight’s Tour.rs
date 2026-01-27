impl Solution {
    /// Solve the knight's tour on an m×n board via backtracking.
    ///
    /// # Intuition
    /// Use DFS with backtracking to visit every cell exactly once starting from (r, c).
    /// The knight moves in 8 possible L-shaped directions.
    ///
    /// # Approach
    /// 1. Initialize the board with -1, mark start as 0
    /// 2. DFS explores all 8 knight moves, backtracking on dead ends
    /// 3. A complete tour fills all m*n cells with step numbers
    ///
    /// # Complexity
    /// - Time: O(8^(m*n)) worst case, pruned heavily in practice
    /// - Space: O(m*n) for the board and recursion stack
    pub fn tour_of_knight(m: i32, n: i32, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m_us, n_us) = (m as usize, n as usize);
        let mut grid: Vec<Vec<i32>> = vec![vec![-1; n_us]; m_us];
        grid[r as usize][c as usize] = 0;
        let dirs: [i32; 9] = [-2, -1, 2, 1, -2, 1, 2, -1, -2];

        fn dfs(
            i: usize,
            j: usize,
            grid: &mut Vec<Vec<i32>>,
            target: i32,
            m: usize,
            n: usize,
            dirs: &[i32; 9],
        ) -> bool {
            if grid[i][j] == target {
                return true;
            }
            for k in 0..8 {
                let x = (i as i32 + dirs[k]) as usize;
                let y = (j as i32 + dirs[k + 1]) as usize;
                if x < m && y < n && grid[x][y] == -1 {
                    grid[x][y] = grid[i][j] + 1;
                    if dfs(x, y, grid, target, m, n, dirs) {
                        return true;
                    }
                    grid[x][y] = -1;
                }
            }
            false
        }

        dfs(r as usize, c as usize, &mut grid, m * n - 1, m_us, n_us, &dirs);
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1x1_board() {
        assert_eq!(Solution::tour_of_knight(1, 1, 0, 0), vec![vec![0]]);
    }

    #[test]
    fn test_small_board() {
        let result = Solution::tour_of_knight(5, 5, 0, 0);
        let n = 5 * 5;
        let mut seen = vec![false; n as usize];
        for row in &result {
            for &val in row {
                assert!(val >= 0 && val < n);
                seen[val as usize] = true;
            }
        }
        assert!(seen.iter().all(|&v| v));
    }
}
