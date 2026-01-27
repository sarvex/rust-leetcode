impl Solution {
    /// Traverses a matrix diagonally in alternating up-right and down-left order.
    ///
    /// # Intuition
    /// On even diagonals (i+j even), move up-right; on odd diagonals, move
    /// down-left. Boundary hits redirect to the next diagonal.
    ///
    /// # Approach
    /// 1. Track position (i, j) and direction via (i+j) parity.
    /// 2. On boundary, advance to the next diagonal start.
    /// 3. Collect m×n elements.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(1) excluding the result
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut i = 0usize;
        let mut j = 0usize;
        (0..m * n)
            .map(|_| {
                let val = mat[i][j];
                if (i + j) % 2 == 0 {
                    if j == n - 1 {
                        i += 1;
                    } else if i == 0 {
                        j += 1;
                    } else {
                        i -= 1;
                        j += 1;
                    }
                } else if i == m - 1 {
                    j += 1;
                } else if j == 0 {
                    i += 1;
                } else {
                    i += 1;
                    j -= 1;
                }
                val
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3x3() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }

    #[test]
    fn test_2x2() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1]]), vec![1]);
    }
}
