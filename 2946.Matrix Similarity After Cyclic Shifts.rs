impl Solution {
    /// Checks matrix identity after k cyclic row shifts.
    ///
    /// # Intuition
    /// A cyclic shift of `k` positions on a row of length `n` is equivalent to
    /// a shift of `k % n`. The row remains unchanged if and only if every element
    /// equals the element `k % n` positions ahead (cyclically). Left-shift by `k`
    /// and right-shift by `k` correspond to offsets `k` and `n − k` respectively.
    ///
    /// # Approach
    /// 1. Compute the effective shift `k % n`.
    /// 2. For each row, pick the offset based on parity: even rows use `k`,
    ///    odd rows use `n − k` (right-shift expressed as a left-shift).
    /// 3. Verify every element equals its shifted counterpart.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(1)
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let n = mat[0].len();
        let k = k as usize % n;
        mat.iter().enumerate().all(|(i, row)| {
            let shift = if i % 2 == 0 { k } else { n - k };
            (0..n).all(|j| row[j] == row[(j + shift) % n])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_similar() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert!(!Solution::are_similar(mat, 4));
    }

    #[test]
    fn test_similar_periodic() {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        assert!(Solution::are_similar(mat, 2));
    }

    #[test]
    fn test_all_equal() {
        let mat = vec![vec![2, 2], vec![2, 2]];
        assert!(Solution::are_similar(mat, 3));
    }

    #[test]
    fn test_single_element_rows() {
        let mat = vec![vec![1], vec![2], vec![3]];
        assert!(Solution::are_similar(mat, 50));
    }

    #[test]
    fn test_k_equals_n() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert!(Solution::are_similar(mat, 3));
    }
}
