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
struct NumMatrix {
    prefix: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut prefix = vec![vec![0; n + 1]; m + 1];
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
}
