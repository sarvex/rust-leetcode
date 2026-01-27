impl Solution {
    /// Constructs a product matrix where each cell holds the product of all other cells mod 12345.
    ///
    /// # Intuition
    /// Similar to "product of array except self", use prefix and suffix products.
    /// Flatten the 2D grid into a linear traversal and accumulate from both ends.
    ///
    /// # Approach
    /// 1. Traverse the grid in reverse, computing suffix products modulo 12345.
    /// 2. Traverse forward, multiplying each cell's suffix value by the running prefix.
    /// 3. Both passes use modular arithmetic with `i64` intermediates to avoid overflow.
    ///
    /// # Complexity
    /// - Time: O(n * m) two passes over the grid
    /// - Space: O(n * m) for the result matrix
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MOD: i64 = 12345;
        let n = grid.len();
        let m = grid[0].len();
        let mut result = vec![vec![0i32; m]; n];

        let mut suf: i64 = 1;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                result[i][j] = suf as i32;
                suf = suf * grid[i][j] as i64 % MOD;
            }
        }

        let mut pre: i64 = 1;
        for i in 0..n {
            for j in 0..m {
                result[i][j] = (result[i][j] as i64 * pre % MOD) as i32;
                pre = pre * grid[i][j] as i64 % MOD;
            }
        }

        result
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
    fn test_single_element() {
        let grid = vec![vec![12345]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, vec![vec![1]]);
    }
}
