impl Solution {
    /// Counts top-left-anchored submatrices with equal frequencies of 'X' and 'Y'.
    ///
    /// # Intuition
    /// Every submatrix containing `grid[0][0]` is uniquely identified by its
    /// bottom-right corner `(i, j)`. We need 2D prefix counts of 'X' and 'Y'
    /// to evaluate each candidate in O(1).
    ///
    /// # Approach
    /// 1. Maintain rolling 1-D prefix arrays for 'X' and 'Y' counts, reusing
    ///    them across rows to achieve O(n) space.
    /// 2. For each row, accumulate a running row sum for both characters, then
    ///    add it to the column prefix. This gives the full 2-D prefix count at
    ///    each cell.
    /// 3. A cell `(i, j)` contributes to the answer when the prefix counts of
    ///    'X' and 'Y' are equal and the 'X' count is positive.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(n)
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid[0].len();
        let mut prefix_x = vec![0_i32; n];
        let mut prefix_y = vec![0_i32; n];
        let mut count = 0;

        for row in &grid {
            let (mut rx, mut ry) = (0, 0);
            for (j, ch) in row.iter().enumerate() {
                rx += i32::from(*ch == 'X');
                ry += i32::from(*ch == 'Y');
                prefix_x[j] += rx;
                prefix_y[j] += ry;
                if prefix_x[j] == prefix_y[j] && prefix_x[j] > 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']];
        assert_eq!(Solution::number_of_submatrices(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec!['X', 'X'], vec!['X', 'Y']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec!['.', '.'], vec!['.', '.']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);
    }

    #[test]
    fn test_single_x() {
        let grid = vec![vec!['X']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);
    }

    #[test]
    fn test_single_xy() {
        let grid = vec![vec!['X', 'Y']];
        assert_eq!(Solution::number_of_submatrices(grid), 1);
    }

    #[test]
    fn test_column_xy() {
        let grid = vec![vec!['X'], vec!['Y']];
        assert_eq!(Solution::number_of_submatrices(grid), 1);
    }

    #[test]
    fn test_all_x() {
        let grid = vec![vec!['X', 'X'], vec!['X', 'X']];
        assert_eq!(Solution::number_of_submatrices(grid), 0);
    }

    #[test]
    fn test_alternating() {
        let grid = vec![vec!['X', 'Y', 'X'], vec!['Y', 'X', 'Y']];
        // (0,1): X=1,Y=1 ✓  (0,2): X=2,Y=1 ✗
        // (1,0): X=1,Y=1 ✓  (1,1): X=2,Y=2 ✓  (1,2): X=3,Y=3 ✓
        assert_eq!(Solution::number_of_submatrices(grid), 4);
    }
}
