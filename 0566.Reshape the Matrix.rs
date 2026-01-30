
impl Solution {
    /// Reshapes a matrix to the given dimensions if element count matches.
    ///
    /// # Intuition
    /// Flatten the matrix conceptually using row-major indexing and redistribute
    /// elements into the new shape.
    ///
    /// # Approach
    /// 1. Verify m×n == r×c; if not, return the original matrix.
    /// 2. Iterate through all elements using a linear index, mapping to new (row, col).
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(r × c)
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let (r, c) = (r as usize, c as usize);
        if m * n != r * c {
            return mat;
        }
        (0..r)
            .map(|i| {
                (0..c)
                    .map(|j| {
                        let idx = i * c + j;
                        mat[idx / n][idx % n]
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reshape() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn test_invalid() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn test_same_shape() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn test_to_column() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2, 3, 4]], 4, 1),
            vec![vec![1], vec![2], vec![3], vec![4]]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::matrix_reshape(vec![vec![1]], 1, 1), vec![vec![1]]);
    }

    #[test]
    fn test_3x2_to_2x3() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 3),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }
}
