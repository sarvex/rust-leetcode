struct NumMatrix {
    prefix: Vec<Vec<i32>>,
}

impl NumMatrix {
    /// 2D immutable range sum query using prefix sums.
    ///
    /// # Intuition
    /// A 2D prefix sum allows O(1) computation of any rectangular sub-region sum
    /// using inclusion-exclusion.
    ///
    /// # Approach
    /// 1. Build a 2D prefix sum matrix of size (m+1) x (n+1).
    /// 2. Query using: prefix[r2+1][c2+1] - prefix[r2+1][c1] - prefix[r1][c2+1] + prefix[r1][c1].
    ///
    /// # Complexity
    /// - Time: O(m * n) construction, O(1) per query
    /// - Space: O(m * n)
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Self {
                prefix: vec![vec![0]],
            };
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prefix = Vec::with_capacity(m + 1);
        prefix.resize(m + 1, vec![0; n + 1]);
        for i in 0..m {
            for j in 0..n {
                prefix[i + 1][j + 1] =
                    prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + matrix[i][j];
            }
        }
        Self { prefix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        self.prefix[r2 + 1][c2 + 1] - self.prefix[r2 + 1][c1] - self.prefix[r1][c2 + 1]
            + self.prefix[r1][c1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_queries() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn single_cell() {
        let matrix = NumMatrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(matrix.sum_region(0, 0, 0, 0), 1);
        assert_eq!(matrix.sum_region(1, 1, 1, 1), 5);
        assert_eq!(matrix.sum_region(2, 2, 2, 2), 9);
    }

    #[test]
    fn full_matrix() {
        let matrix = NumMatrix::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(matrix.sum_region(0, 0, 1, 1), 10);
    }

    #[test]
    fn single_row() {
        let matrix = NumMatrix::new(vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(matrix.sum_region(0, 0, 0, 4), 15);
        assert_eq!(matrix.sum_region(0, 1, 0, 3), 9);
    }

    #[test]
    fn single_column() {
        let matrix = NumMatrix::new(vec![vec![1], vec![2], vec![3]]);
        assert_eq!(matrix.sum_region(0, 0, 2, 0), 6);
    }

    #[test]
    fn negative_values() {
        let matrix = NumMatrix::new(vec![vec![-1, -2], vec![-3, -4]]);
        assert_eq!(matrix.sum_region(0, 0, 1, 1), -10);
    }
}
