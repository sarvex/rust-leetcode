impl Solution {
    /// Simulates gravity per row, then rotates 90° clockwise in one pass.
    ///
    /// # Intuition
    /// Rotating 90° clockwise maps `(r, c)` in the original `m x n` grid to
    /// `(c, m - 1 - r)` in the resulting `n x m` grid. Gravity after rotation
    /// pulls stones in the direction of increasing column of the original
    /// grid (they fall "to the right" before rotation). Obstacles `*` are
    /// fixed anchors; empty cells `.` are where stones slide through.
    ///
    /// # Approach
    /// For each row of the input:
    /// - Scan from right to left, tracking `write`, the next slot a stone can
    ///   occupy.
    /// - When we hit an obstacle `*`, place it at its original position in the
    ///   rotated output and reset `write` to just left of it.
    /// - When we hit a stone `#`, place it at column `write` in the row, then
    ///   decrement `write`.
    /// - Empty cells are the default `.` fill in the output.
    ///
    /// Each placement is written directly at its rotated coordinate
    /// `(original_col, m - 1 - original_row)`, avoiding a separate rotation
    /// pass.
    ///
    /// # Complexity
    /// - Time: O(m * n) — each cell is visited once.
    /// - Space: O(m * n) — for the output grid (excluding input).
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        let mut out = vec![vec!['.'; m]; n];

        for (r, row) in box_grid.iter().enumerate() {
            let rotated_col = m - 1 - r;
            let mut write = n - 1;
            for c in (0..n).rev() {
                match row[c] {
                    '*' => {
                        out[c][rotated_col] = '*';
                        write = c.saturating_sub(1);
                    }
                    '#' => {
                        out[write][rotated_col] = '#';
                        write = write.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_row() {
        let input = vec![vec!['#', '.', '#']];
        let expected = vec![vec!['.'], vec!['#'], vec!['#']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_two_rows_with_obstacle() {
        let input = vec![
            vec!['#', '.', '*', '.'],
            vec!['#', '#', '*', '.'],
        ];
        let expected = vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_three_rows_mixed() {
        let input = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let expected = vec![
            vec!['.', '#', '#'],
            vec!['.', '#', '#'],
            vec!['#', '#', '*'],
            vec!['#', '*', '.'],
            vec!['#', '.', '*'],
            vec!['#', '.', '.'],
        ];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_all_empty() {
        let input = vec![vec!['.', '.', '.']];
        let expected = vec![vec!['.'], vec!['.'], vec!['.']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_all_obstacles() {
        let input = vec![vec!['*', '*', '*']];
        let expected = vec![vec!['*'], vec!['*'], vec!['*']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_single_cell_stone() {
        let input = vec![vec!['#']];
        let expected = vec![vec!['#']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_single_cell_empty() {
        let input = vec![vec!['.']];
        let expected = vec![vec!['.']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_stones_blocked_by_obstacle() {
        // Stones should not cross obstacle to the right.
        let input = vec![vec!['#', '#', '*', '.', '.']];
        let expected = vec![
            vec!['#'],
            vec!['#'],
            vec!['*'],
            vec!['.'],
            vec!['.'],
        ];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }

    #[test]
    fn test_stones_fall_to_bottom_after_rotation() {
        // After rotation, a single column of stones with no obstacles falls
        // to the bottom (right side of original row).
        let input = vec![vec!['#', '.', '.', '.']];
        let expected = vec![vec!['.'], vec!['.'], vec!['.'], vec!['#']];
        assert_eq!(Solution::rotate_the_box(input), expected);
    }
}
