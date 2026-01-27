impl Solution {
    /// Computes the difference matrix of ones and zeros in rows and columns.
    ///
    /// # Intuition
    /// For cell (i,j), diff = onesRow_i + onesCol_j - zerosRow_i - zerosCol_j.
    /// Since zerosRow = n - onesRow, the formula simplifies to
    /// 2*onesRow + 2*onesCol - m - n.
    ///
    /// # Approach
    /// 1. Count ones per row and per column in a single pass
    /// 2. Build the result using the simplified formula
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m + n) for row/column counts
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_ones = vec![0i32; m];
        let mut col_ones = vec![0i32; n];

        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                row_ones[i] += val;
                col_ones[j] += val;
            }
        }

        let (m_i32, n_i32) = (m as i32, n as i32);
        (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| 2 * row_ones[i] + 2 * col_ones[j] - m_i32 - n_i32)
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let expected = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn test_all_ones() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![4, 4], vec![4, 4]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn test_all_zeros() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        let expected = vec![vec![-4, -4], vec![-4, -4]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }
}
