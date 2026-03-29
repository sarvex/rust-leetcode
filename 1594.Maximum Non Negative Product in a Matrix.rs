impl Solution {
    /// Grid DP tracking both maximum and minimum products per cell.
    ///
    /// # Intuition
    /// Negative values can flip a minimum product into a maximum when multiplied
    /// by another negative. Tracking both extremes at every cell captures this.
    ///
    /// # Approach
    /// 1. Maintain two DP tables (`dp_max`, `dp_min`) storing the largest and
    ///    smallest product of any path from `(0, 0)` to `(i, j)`.
    /// 2. Initialize the first row and first column by cumulative multiplication.
    /// 3. For interior cells, derive candidates from the top and left neighbors.
    ///    Multiply each neighbor's max and min by `grid[i][j]`, then take the
    ///    overall max and min of all four candidates.
    /// 4. If `dp_max[m-1][n-1]` is negative, no non-negative path exists; return
    ///    `-1`. Otherwise return the value modulo `10^9 + 7`.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n)
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (m, n) = (grid.len(), grid[0].len());

        let mut dp_max = vec![vec![0_i64; n]; m];
        let mut dp_min = vec![vec![0_i64; n]; m];

        dp_max[0][0] = grid[0][0] as i64;
        dp_min[0][0] = grid[0][0] as i64;

        for i in 1..m {
            let v = grid[i][0] as i64;
            dp_max[i][0] = dp_max[i - 1][0] * v;
            dp_min[i][0] = dp_min[i - 1][0] * v;
            if dp_max[i][0] < dp_min[i][0] {
                std::mem::swap(&mut dp_max[i][0], &mut dp_min[i][0]);
            }
        }

        for j in 1..n {
            let v = grid[0][j] as i64;
            dp_max[0][j] = dp_max[0][j - 1] * v;
            dp_min[0][j] = dp_min[0][j - 1] * v;
            if dp_max[0][j] < dp_min[0][j] {
                std::mem::swap(&mut dp_max[0][j], &mut dp_min[0][j]);
            }
        }

        for i in 1..m {
            for j in 1..n {
                let v = grid[i][j] as i64;
                let candidates = [
                    dp_max[i - 1][j] * v,
                    dp_min[i - 1][j] * v,
                    dp_max[i][j - 1] * v,
                    dp_min[i][j - 1] * v,
                ];
                dp_max[i][j] = *candidates.iter().max().unwrap();
                dp_min[i][j] = *candidates.iter().min().unwrap();
            }
        }

        if dp_max[m - 1][n - 1] < 0 {
            -1
        } else {
            (dp_max[m - 1][n - 1] % MOD) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_negative_paths() {
        let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];
        assert_eq!(Solution::max_product_path(grid), -1);
    }

    #[test]
    fn double_negative_flip() {
        let grid = vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]];
        assert_eq!(Solution::max_product_path(grid), 8);
    }

    #[test]
    fn zero_in_path() {
        let grid = vec![vec![1, 3], vec![0, -4]];
        assert_eq!(Solution::max_product_path(grid), 0);
    }

    #[test]
    fn single_cell_positive() {
        assert_eq!(Solution::max_product_path(vec![vec![4]]), 4);
    }

    #[test]
    fn single_cell_negative() {
        assert_eq!(Solution::max_product_path(vec![vec![-3]]), -1);
    }

    #[test]
    fn single_row() {
        let grid = vec![vec![1, -2, -3, 4]];
        assert_eq!(Solution::max_product_path(grid), 24);
    }

    #[test]
    fn single_column() {
        let grid = vec![vec![2], vec![-3], vec![-4]];
        assert_eq!(Solution::max_product_path(grid), 24);
    }
}
