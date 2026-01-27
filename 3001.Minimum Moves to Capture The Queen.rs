impl Solution {
    /// Minimum moves for rook or bishop to capture the queen.
    ///
    /// # Intuition
    /// The rook can capture in one move if it shares a row or column with the queen
    /// and the bishop does not block the path. Similarly, the bishop captures in one
    /// move along a shared diagonal if the rook does not block. Otherwise two moves
    /// always suffice.
    ///
    /// # Approach
    /// 1. Check if rook shares a column with the queen and the bishop is not between them.
    /// 2. Check if rook shares a row with the queen and the bishop is not between them.
    /// 3. Check both diagonals for the bishop with the same blocking logic.
    /// 4. Return 1 for any direct capture, otherwise 2.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        let rook_column = a == e && (c != a || (d - b) * (d - f) > 0);
        let rook_row = b == f && (d != b || (c - a) * (c - e) > 0);
        let bishop_diag1 = c - e == d - f && (a - e != b - f || (a - c) * (a - e) > 0);
        let bishop_diag2 = c - e == f - d && (a - e != f - b || (a - c) * (a - e) > 0);

        match rook_column || rook_row || bishop_diag1 || bishop_diag2 {
            true => 1,
            false => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rook_captures_queen_on_same_row() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 1),
            1
        );
    }

    #[test]
    fn bishop_captures_queen_on_diagonal() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(5, 3, 3, 1, 1, 3),
            1
        );
    }

    #[test]
    fn requires_two_moves() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(4, 3, 3, 4, 5, 2),
            2
        );
    }
}
