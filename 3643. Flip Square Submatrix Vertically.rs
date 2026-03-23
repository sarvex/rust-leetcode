impl Solution {
    /// Flip a square submatrix vertically by reversing row order.
    ///
    /// # Intuition
    /// We need to reverse the rows within a square submatrix while keeping
    /// the rest of the grid unchanged. The submatrix is defined by its
    /// top-left corner (x, y) and side length k.
    ///
    /// # Approach
    /// 1. Clone the grid to avoid modifying the original
    /// 2. For each row in the submatrix (k rows total), swap rows symmetrically
    ///    from the top and bottom of the submatrix
    /// 3. Return the modified grid
    ///
    /// # Complexity
    /// - Time: O(k²) where k is the side length of the submatrix
    /// - Space: O(m × n) for the cloned grid
    ///
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        // Reverse rows within the submatrix
        // Row at position (x + i) swaps with row at position (x + k - 1 - i)
        for i in 0..k / 2 {
            let top_row = x + i;
            let bottom_row = x + k - 1 - i;

            // Swap the elements within the column range [y, y + k)
            for j in y..y + k {
                grid[top_row][j] = grid[top_row][j] ^ grid[bottom_row][j];
                grid[bottom_row][j] = grid[top_row][j] ^ grid[bottom_row][j];
                grid[top_row][j] = grid[top_row][j] ^ grid[bottom_row][j];
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let result = Solution::reverse_submatrix(grid, 1, 0, 3);
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![13, 14, 15, 8],
            vec![9, 10, 11, 12],
            vec![5, 6, 7, 16],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]];
        let result = Solution::reverse_submatrix(grid, 0, 2, 2);
        let expected = vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        let grid = vec![vec![5]];
        let result = Solution::reverse_submatrix(grid, 0, 0, 1);
        let expected = vec![vec![5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_k_equals_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::reverse_submatrix(grid, 0, 0, 2);
        let expected = vec![vec![4, 5, 3], vec![1, 2, 6], vec![7, 8, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_at_corner() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::reverse_submatrix(grid, 0, 0, 3);
        let expected = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_submatrix_in_middle() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let result = Solution::reverse_submatrix(grid, 1, 1, 2);
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![5, 10, 9, 8],
            vec![9, 6, 7, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_larger_grid() {
        let grid = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let result = Solution::reverse_submatrix(grid, 1, 1, 3);
        let expected = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 16, 17, 18, 10],
            vec![11, 11, 12, 13, 15],
            vec![16, 7, 8, 9, 20],
            vec![21, 22, 23, 24, 25],
        ];
        assert_eq!(result, expected);
    }
}
