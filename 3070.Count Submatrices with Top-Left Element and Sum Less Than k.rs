impl Solution {
    /// Counts submatrices anchored at top-left with sum ≤ k using 2D prefix sums.
    ///
    /// # Intuition
    /// Every submatrix containing the top-left element is uniquely defined by its
    /// bottom-right corner (i, j). The sum of such a submatrix equals the 2D prefix
    /// sum at (i, j). We compute prefix sums in-place and count entries ≤ k.
    ///
    /// # Approach
    /// 1. Build a 2D prefix sum by accumulating row-wise, then column-wise.
    /// 2. For each cell, check if the prefix sum is ≤ k and increment the count.
    /// 3. Early-break on each row once the prefix sum exceeds k, since all wider
    ///    submatrices in that row will also exceed k (values are non-negative).
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(1) extra (prefix sums computed in-place)
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    grid[i][j] += grid[i - 1][j];
                }
                if j > 0 {
                    grid[i][j] += grid[i][j - 1];
                }
                if i > 0 && j > 0 {
                    grid[i][j] -= grid[i - 1][j - 1];
                }
                if grid[i][j] <= k {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
        assert_eq!(Solution::count_submatrices(grid, 18), 4);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        assert_eq!(Solution::count_submatrices(grid, 20), 6);
    }

    #[test]
    fn test_single_element_pass() {
        let grid = vec![vec![5]];
        assert_eq!(Solution::count_submatrices(grid, 5), 1);
    }

    #[test]
    fn test_single_element_fail() {
        let grid = vec![vec![10]];
        assert_eq!(Solution::count_submatrices(grid, 5), 0);
    }

    #[test]
    fn test_all_zeros() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::count_submatrices(grid, 0), 4);
    }

    #[test]
    fn test_large_k() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::count_submatrices(grid, 1_000_000_000), 4);
    }
}
