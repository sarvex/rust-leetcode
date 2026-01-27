impl Solution {
    /// Two-pass marker approach for setting matrix rows and columns to zero.
    ///
    /// # Intuition
    /// First identify which rows and columns contain a zero, then apply
    /// the zeroing in a second pass. Boolean marker arrays avoid modifying
    /// the matrix prematurely.
    ///
    /// # Approach
    /// Scan the matrix to record which rows and columns contain zeros in
    /// boolean vectors. Then iterate again, setting any cell to zero if
    /// its row or column is marked.
    ///
    /// # Complexity
    /// - Time: O(m × n) — two full scans
    /// - Space: O(m + n) — marker arrays for rows and columns
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut zero_rows = vec![false; m];
        let mut zero_cols = vec![false; n];

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    zero_rows[i] = true;
                    zero_cols[j] = true;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if zero_rows[i] || zero_cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn multiple_zeros() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }

    #[test]
    fn no_zeros() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 2], vec![3, 4]]);
    }
}
