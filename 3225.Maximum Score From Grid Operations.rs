impl Solution {
    /// Dynamic programming over column heights with prefix-sum scoring.
    ///
    /// # Intuition
    /// Each column ends up with a black prefix of some height. A white cell contributes to the score
    /// exactly when a neighbor column is black at the same row, which depends only on the maximum
    /// black height of the two neighbors.
    ///
    /// # Approach
    /// Treat each column as a height `h_j` in `[0, n]` (black rows `0..h_j`). For column `j`,
    /// a white row `i >= h_j` counts if `i < max(h_{j-1}, h_{j+1})`, so the column contribution
    /// is the sum of `grid[i][j]` for `i` in `[h_j, max(h_{j-1}, h_{j+1}) - 1]`. Using per-column
    /// prefix sums, this contribution is `prefix[j][max] - prefix[j][h_j]`.
    ///
    /// DP keeps a state of two adjacent heights `(h_{j-1}, h_j)`. When choosing `h_{j+1}`, we can
    /// finalize the contribution of column `j`. Initialize with column `0` (left neighbor height 0),
    /// transition through columns `1..n-2`, then add the last column using right neighbor height 0.
    ///
    /// # Complexity
    /// - Time: O(n^4) via O(n * (n + 1)^3) transitions
    /// - Space: O(n^2) for the DP table
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n == 1 {
            return 0;
        }

        let height_count = n + 1;
        let mut col_prefix: Vec<Vec<i64>> = Vec::with_capacity(n);
        for col in 0..n {
            let mut prefix = vec![0_i64; n + 1];
            for row in 0..n {
                prefix[row + 1] = prefix[row] + grid[row][col] as i64;
            }
            col_prefix.push(prefix);
        }

        let column_sum = |col: usize, start: usize, end: usize| -> i64 {
            col_prefix[col][end] - col_prefix[col][start]
        };

        let score_for_column = |col: usize, self_height: usize, neighbor_max: usize| -> i64 {
            if self_height >= neighbor_max {
                0
            } else {
                column_sum(col, self_height, neighbor_max)
            }
        };

        let neg_inf = i64::MIN / 4;
        let mut dp = vec![vec![neg_inf; height_count]; height_count];

        for h0 in 0..height_count {
            for h1 in 0..height_count {
                dp[h0][h1] = score_for_column(0, h0, h1);
            }
        }

        for col in 1..=n - 2 {
            let mut next_dp = vec![vec![neg_inf; height_count]; height_count];
            for prev in 0..height_count {
                for curr in 0..height_count {
                    let base = dp[prev][curr];
                    if base == neg_inf {
                        continue;
                    }
                    for next in 0..height_count {
                        let neighbor_max = if prev > next { prev } else { next };
                        let candidate = base + score_for_column(col, curr, neighbor_max);
                        if candidate > next_dp[curr][next] {
                            next_dp[curr][next] = candidate;
                        }
                    }
                }
            }
            dp = next_dp;
        }

        let mut best = 0_i64;
        for prev in 0..height_count {
            for curr in 0..height_count {
                let base = dp[prev][curr];
                if base == neg_inf {
                    continue;
                }
                let candidate = base + score_for_column(n - 1, curr, prev);
                if candidate > best {
                    best = candidate;
                }
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 3, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![5, 0, 0, 3, 0],
            vec![0, 0, 0, 0, 2],
        ];
        assert_eq!(Solution::maximum_score(grid), 11);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![10, 9, 0, 0, 15],
            vec![7, 1, 0, 8, 0],
            vec![5, 20, 0, 11, 0],
            vec![0, 0, 0, 1, 2],
            vec![8, 12, 1, 10, 3],
        ];
        assert_eq!(Solution::maximum_score(grid), 94);
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![5]];
        assert_eq!(Solution::maximum_score(grid), 0);
    }

    #[test]
    fn test_two_columns_simple() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::maximum_score(grid), 6);
    }
}
