impl Solution {
    /// Captures surrounded regions using border-connected DFS marking.
    ///
    /// # Intuition
    /// Any 'O' connected to the border cannot be captured. Mark all border-connected
    /// 'O' cells first, then flip the remaining 'O' cells to 'X'.
    ///
    /// # Approach
    /// 1. DFS from every border 'O', marking reachable cells with a sentinel '.'.
    /// 2. Iterate through the entire board:
    ///    - Restore '.' cells back to 'O' (border-connected).
    ///    - Flip remaining 'O' cells to 'X' (surrounded).
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n) worst-case recursion stack
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        if m == 0 {
            return;
        }
        let n = board[0].len();

        for i in 0..m {
            Self::dfs(board, i, 0, m, n);
            Self::dfs(board, i, n - 1, m, n);
        }
        for j in 0..n {
            Self::dfs(board, 0, j, m, n);
            Self::dfs(board, m - 1, j, m, n);
        }

        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                match *cell {
                    '.' => *cell = 'O',
                    'O' => *cell = 'X',
                    _ => {}
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        if i >= m || j >= n || board[i][j] != 'O' {
            return;
        }
        board[i][j] = '.';
        if i + 1 < m {
            Self::dfs(board, i + 1, j, m, n);
        }
        if i > 0 {
            Self::dfs(board, i - 1, j, m, n);
        }
        if j + 1 < n {
            Self::dfs(board, i, j + 1, m, n);
        }
        if j > 0 {
            Self::dfs(board, i, j - 1, m, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_surrounded_region() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }

    #[test]
    fn single_cell() {
        let mut board = vec![vec!['O']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['O']]);
    }
}
