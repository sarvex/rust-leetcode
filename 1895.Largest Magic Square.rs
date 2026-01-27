impl Solution {
    /// Finds the largest magic square using prefix sums with early termination.
    ///
    /// # Intuition
    /// Use row and column prefix sums for O(1) sum queries. Check squares from
    /// largest to smallest with aggressive early termination.
    ///
    /// # Approach
    /// 1. Build prefix sums for rows and columns.
    /// 2. For each size from largest down to 2, enumerate all positions.
    /// 3. Compare first row vs first column to prune fast.
    /// 4. Check diagonals before remaining rows and columns.
    ///
    /// # Complexity
    /// - Time: O(m * n * min(m, n)^2)
    /// - Space: O(m * n)
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut row_sum = vec![vec![0i64; n + 1]; m];
        for i in 0..m {
            for j in 0..n {
                row_sum[i][j + 1] = row_sum[i][j] + grid[i][j] as i64;
            }
        }

        let mut col_sum = vec![vec![0i64; n]; m + 1];
        for i in 0..m {
            for j in 0..n {
                col_sum[i + 1][j] = col_sum[i][j] + grid[i][j] as i64;
            }
        }

        let get_row = |i: usize, j: usize, k: usize| row_sum[i][j + k] - row_sum[i][j];
        let get_col = |i: usize, j: usize, k: usize| col_sum[i + k][j] - col_sum[i][j];

        for edge in (2..=m.min(n)).rev() {
            for i in 0..=m - edge {
                'inner: for j in 0..=n - edge {
                    let target = get_row(i, j, edge);

                    if get_col(i, j, edge) != target {
                        continue;
                    }

                    let (mut d1, mut d2) = (0i64, 0i64);
                    for k in 0..edge {
                        d1 += grid[i + k][j + k] as i64;
                        d2 += grid[i + k][j + edge - 1 - k] as i64;
                    }
                    if d1 != target || d2 != target {
                        continue;
                    }

                    for ii in 1..edge {
                        if get_row(i + ii, j, edge) != target {
                            continue 'inner;
                        }
                    }

                    for jj in 1..edge {
                        if get_col(i, j + jj, edge) != target {
                            continue 'inner;
                        }
                    }

                    return edge as i32;
                }
            }
        }

        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let grid = vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4],
        ];
        assert_eq!(Solution::largest_magic_square(grid), 3);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(
            Solution::largest_magic_square(vec![
                vec![5, 1, 3, 1],
                vec![9, 3, 3, 1],
                vec![1, 3, 3, 8]
            ]),
            2
        );
    }

    #[test]
    fn test_single_cell() {
        assert_eq!(Solution::largest_magic_square(vec![vec![1]]), 1);
    }

    #[test]
    fn test_uniform_matrix() {
        assert_eq!(
            Solution::largest_magic_square(vec![vec![1, 1], vec![1, 1]]),
            2
        );
    }
}
