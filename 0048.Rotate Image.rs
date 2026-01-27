impl Solution {
    /// Transpose-then-reverse for in-place 90-degree clockwise rotation.
    ///
    /// # Intuition
    /// A 90° clockwise rotation can be decomposed into a vertical flip
    /// (reverse rows) followed by a transpose (swap across the main diagonal).
    /// Both operations are O(n^2) and in-place.
    ///
    /// # Approach
    /// First reverse the row order (swap row `i` with row `n-1-i`).
    /// Then transpose the matrix by swapping `matrix[i][j]` with
    /// `matrix[j][i]` for all `j < i`.
    ///
    /// # Complexity
    /// - Time: O(n^2) — two passes over the matrix
    /// - Space: O(1) — in-place swaps
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n / 2 {
            for j in 0..n {
                let (a, b) = (i, n - i - 1);
                let tmp = matrix[a][j];
                matrix[a][j] = matrix[b][j];
                matrix[b][j] = tmp;
            }
        }

        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn four_by_four() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ]
        );
    }

    #[test]
    fn single_element() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }
}
