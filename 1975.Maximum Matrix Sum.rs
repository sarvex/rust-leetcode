impl Solution {
    /// Maximizes matrix sum by optimally negating adjacent pairs.
    ///
    /// # Intuition
    /// Adjacent negation can move a negative sign anywhere. With an even
    /// count of negatives, all values become positive. With odd count,
    /// the smallest absolute value must remain negative.
    ///
    /// # Approach
    /// 1. Sum absolute values and count negatives.
    /// 2. Track the minimum absolute value.
    /// 3. If negative count is odd, subtract twice the minimum.
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(1)
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (total, min_abs, neg_count) = matrix.iter().flat_map(|row| row.iter()).fold(
            (0i64, i32::MAX, 0),
            |(total, min_abs, neg_count), &val| {
                let abs = val.abs();
                (
                    total + abs as i64,
                    min_abs.min(abs),
                    neg_count + i32::from(val < 0),
                )
            },
        );

        if neg_count % 2 == 0 {
            total
        } else {
            total - 2 * min_abs as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_negatives() {
        assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
    }

    #[test]
    fn test_odd_negatives() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );
    }
}
