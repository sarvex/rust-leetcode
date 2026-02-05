impl Solution {
    /// In-place marker approach using the first row and column.
    ///
    /// # Intuition
    /// Use the first row and first column as marker storage to track which
    /// rows and columns should be zeroed, while keeping two flags for whether
    /// the first row or column originally contained a zero.
    ///
    /// # Approach
    /// Scan the inner submatrix to write markers into the first row/column.
    /// Then zero cells based on those markers and finally zero the first
    /// row/column if their original flags were set.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(1) — in-place markers plus constant flags
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let first_row_has_zero = matrix[0].iter().any(|value| *value == 0);
        let first_col_has_zero = matrix.iter().any(|row| row[0] == 0);

        (1..m).for_each(|i| {
            (1..n).filter(|&j| matrix[i][j] == 0).for_each(|j| {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            });
        });

        (1..m).for_each(|i| {
            let zero_row = matrix[i][0] == 0;
            (1..n)
                .filter(|&j| zero_row || matrix[0][j] == 0)
                .for_each(|j| matrix[i][j] = 0);
        });

        if first_row_has_zero {
            matrix[0].fill(0);
        }

        if first_col_has_zero {
            matrix.iter_mut().for_each(|row| row[0] = 0);
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
