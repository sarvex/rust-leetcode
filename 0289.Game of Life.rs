impl Solution {
    /// Simulates one step of Conway's Game of Life using a neighbor count matrix.
    ///
    /// # Intuition
    /// Count live neighbors for each cell, then apply the rules simultaneously
    /// using a separate weight matrix to avoid in-place conflicts.
    ///
    /// # Approach
    /// 1. Build a neighbor count matrix by iterating over all live cells.
    /// 2. Apply the rules:
    ///    - Live cell with < 2 or > 3 neighbors dies.
    ///    - Dead cell with exactly 3 neighbors becomes alive.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n) for the weight matrix
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        let mut neighbors = vec![vec![0i32; n]; m];

        const DIRS: [(i32, i32); 8] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 1 {
                    for (di, dj) in &DIRS {
                        let (ni, nj) = (i as i32 + di, j as i32 + dj);
                        if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                            neighbors[ni as usize][nj as usize] += 1;
                        }
                    }
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                match neighbors[i][j] {
                    3 => {
                        if board[i][j] == 0 {
                            board[i][j] = 1;
                        }
                    }
                    2..=3 => {}
                    _ => board[i][j] = 0,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_board() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
    }
}
