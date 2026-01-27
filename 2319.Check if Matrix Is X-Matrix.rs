impl Solution {
    /// Checks whether a matrix is an X-Matrix.
    ///
    /// # Intuition
    /// An X-Matrix has non-zero values only on both diagonals and zeros elsewhere.
    /// Each cell can be checked independently against its diagonal membership.
    ///
    /// # Approach
    /// Iterate all cells, checking the diagonal condition: a cell (i, j) is on a
    /// diagonal if `i == j` or `i + j == n - 1`. Verify non-zero for diagonals
    /// and zero for all other positions.
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(1)
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        (0..n).all(|i| {
            (0..n).all(|j| {
                let on_diagonal = i == j || i + j == n - 1;
                if on_diagonal {
                    grid[i][j] != 0
                } else {
                    grid[i][j] == 0
                }
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_x_matrix() {
        let grid = vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];
        assert!(Solution::check_x_matrix(grid));
    }

    #[test]
    fn test_invalid_zero_on_diagonal() {
        let grid = vec![
            vec![5, 0, 0, 1],
            vec![0, 0, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];
        assert!(!Solution::check_x_matrix(grid));
    }

    #[test]
    fn test_invalid_nonzero_off_diagonal() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert!(!Solution::check_x_matrix(grid));
    }

    #[test]
    fn test_single_element() {
        assert!(Solution::check_x_matrix(vec![vec![5]]));
    }
}
