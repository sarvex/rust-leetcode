impl Solution {
    /// Constructs a product matrix where each cell holds the product of all other cells mod 12345.
    ///
    /// # Intuition
    /// Similar to "product of array except self", use prefix and suffix products.
    /// Flatten the 2D grid into a contiguous suffix array for cache-friendly access,
    /// then combine with a running prefix in a single forward pass that overwrites
    /// the grid in place.
    ///
    /// # Approach
    /// 1. Build a flat suffix-product array indexed by linearised position.
    /// 2. Forward-pass: read the original cell value, overwrite the cell with
    ///    `prefix * suffix[pos + 1] mod 12345`, then advance the prefix.
    /// 3. Reuse the input grid as the output to avoid a second `Vec<Vec>` allocation.
    ///
    /// # Complexity
    /// - Time: O(n * m) two passes over the grid
    /// - Space: O(n * m) for the flat suffix array (grid reused, no extra matrix)
    pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;
        let n = grid.len();
        let m = grid[0].len();
        let total = n * m;

        let mut suf = vec![1_i64; total + 1];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                let pos = i * m + j;
                suf[pos] = suf[pos + 1] * grid[i][j] as i64 % MOD;
            }
        }

        let mut pre = 1_i64;
        for i in 0..n {
            for j in 0..m {
                let val = grid[i][j] as i64;
                grid[i][j] = (pre * suf[i * m + j + 1] % MOD) as i32;
                pre = pre * val % MOD;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_grid() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, vec![vec![24, 12], vec![8, 6]]);
    }

    #[test]
    fn test_modulo_zeros() {
        let grid = vec![vec![12345], vec![2], vec![1]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, vec![vec![2], vec![0], vec![0]]);
    }

    #[test]
    fn test_single_element() {
        let grid = vec![vec![12345]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![1, 2, 3, 4, 5]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, vec![vec![120, 60, 40, 30, 24]]);
    }

    #[test]
    fn test_large_values() {
        let grid = vec![vec![1_000_000_000, 1_000_000_000]];
        let result = Solution::construct_product_matrix(grid);
        let expected = (1_000_000_000_i64 % 12345) as i32;
        assert_eq!(result, vec![vec![expected, expected]]);
    }
}
