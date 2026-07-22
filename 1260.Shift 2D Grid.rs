impl Solution {
    /// Shift 2D grid k times by flattening, rotating, and reshaping.
    ///
    /// # Intuition
    /// A 2D grid shift is equivalent to a cyclic rotation of the underlying
    /// flat sequence. Flattening to 1D lets us apply a single modular offset
    /// to map every source index to its destination in O(m·n) time.
    ///
    /// # Approach
    /// 1. Compute `total = m * n` and reduce `k %= total` to avoid redundant work.
    /// 2. For each destination position `pos` in `[0, total)`, the source index is
    ///    `(pos + total - k) % total`. Map 1D indices back to 2D with `/` and `%`.
    /// 3. Collect directly into the result grid — no intermediate flat vector needed.
    ///
    /// # Complexity
    /// - Time: O(m·n)
    /// - Space: O(m·n) for the output grid
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;
        let shift = k as usize % total;

        (0..m)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let src = (i * n + j + total - shift) % total;
                        grid[src / n][src % n]
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
    fn test_example1() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::shift_grid(grid, 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn test_example2() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        assert_eq!(
            Solution::shift_grid(grid, 4),
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
            ]
        );
    }

    #[test]
    fn test_full_rotation() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = grid.clone();
        assert_eq!(Solution::shift_grid(grid, 9), expected);
    }

    #[test]
    fn test_k_zero() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let expected = grid.clone();
        assert_eq!(Solution::shift_grid(grid, 0), expected);
    }

    #[test]
    fn test_single_row() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3, 4, 5]], 2),
            vec![vec![4, 5, 1, 2, 3]]
        );
    }

    #[test]
    fn test_single_column() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1], vec![2], vec![3]], 1),
            vec![vec![3], vec![1], vec![2]]
        );
    }

    #[test]
    fn test_k_larger_than_total() {
        // k=100, total=9 → effective shift = 100 % 9 = 1, same as example1
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::shift_grid(grid, 100),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }
}
