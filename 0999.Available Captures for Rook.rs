impl Solution {
    /// Counts pawns capturable by the rook on a chess board.
    ///
    /// # Intuition
    /// Find the rook's position, then scan in four cardinal directions until
    /// hitting a bishop (blocked) or pawn (captured) or board edge.
    ///
    /// # Approach
    /// Locate 'R' on the board. For each direction, walk until blocked by 'B',
    /// capturing 'p' if encountered first.
    ///
    /// # Complexity
    /// - Time: O(64) = O(1) for 8x8 board
    /// - Space: O(1)
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let (mut rr, mut rc) = (0, 0);
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    rr = i;
                    rc = j;
                }
            }
        }

        let mut captures = 0;
        for (dr, dc) in [(!0usize, 0), (1, 0), (0, !0usize), (0, 1)] {
            let (mut r, mut c) = (rr, rc);
            loop {
                r = r.wrapping_add(dr);
                c = c.wrapping_add(dc);
                if r >= 8 || c >= 8 || board[r][c] == 'B' {
                    break;
                }
                if board[r][c] == 'p' {
                    captures += 1;
                    break;
                }
            }
        }
        captures
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board), 3);
    }

    #[test]
    fn test_blocked_by_bishop() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board), 0);
    }
}
