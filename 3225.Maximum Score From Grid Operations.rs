impl Solution {
    /// Dynamic programming over column heights with three rolling states.
    ///
    /// # Intuition
    /// Each column ends up with a black prefix of height `h_j ∈ [0, n]`. A white cell `(i, j)`
    /// (where `i >= h_j`) contributes `grid[i][j]` iff `i < max(h_{j-1}, h_{j+1})`. Naively
    /// tracking a pair of adjacent heights yields O(n^4). By splitting each column's contribution
    /// into a "left-covered" part and a "right-covered" part, and by maintaining three carefully
    /// chosen rolling states (indexed by the current column's height), the transition per column
    /// becomes O(n^2), giving an overall O(n^3) algorithm.
    ///
    /// # Approach
    /// Process columns left to right. For each column `i`, maintain three arrays indexed by the
    /// height `h ∈ [0, n]` of column `i`:
    ///   * `dp1[h]` — best score when column `i`'s scoring window is entirely supplied by its left
    ///     neighbor (the left column's height already reaches or exceeds the top of the window).
    ///   * `dp2[h]` — best score when column `i`'s window is partially pending and its right
    ///     neighbor will finish covering the remaining white rows.
    ///   * `dp3[h]` — prefix-max over `dp2`-style values up to height `h`, used to cheaply query
    ///     the best previous state whose height is at most a given threshold.
    ///
    /// During the transition into column `i`, iterate the chosen height `j` from `n-1` down to
    /// `0`. For every candidate previous-column height `k ∈ [0, j]` maintain `s2` — the suffix
    /// sum of column `i` starting at row `k+1` — so extending the window by one row costs O(1).
    /// The best of `dp1[k] + s2`, `dp3[j] + s2`, and a running `pre` value updates the next
    /// column's three DP arrays. `pre` tracks the best attainable value when the previous column
    /// was taller than the current one and is also updated in O(1) per step.
    ///
    /// The final answer is the maximum value observed across all reachable DP states.
    ///
    /// # Complexity
    /// - Time: O(n^3)
    /// - Space: O(n)
    #[allow(clippy::needless_range_loop)]
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        if n <= 1 {
            return 0;
        }

        let mut dp1 = vec![0_i64; n];
        let mut dp2 = vec![0_i64; n + 1];
        let mut dp3 = vec![0_i64; n + 1];
        let mut dp1_next = vec![0_i64; n];
        let mut dp2_next = vec![0_i64; n + 1];
        let mut dp3_next = vec![0_i64; n + 1];
        let mut res = 0_i64;

        for i in 0..n {
            // `sum` tracks the running suffix total of column `i` from row 0 to row n-1.
            // It is decremented as `j` decreases so we can reuse it across the inner loop.
            let mut sum: i64 = (0..n).map(|r| grid[r][i] as i64).sum();
            let mut pre = 0_i64;

            for j in (0..n).rev() {
                // `s2` will iterate through suffix sums of column `i` starting at row `k+1`.
                let mut s2 = sum;
                // Baseline for dp1_next[j]: previous column at height 0 contributes nothing, and
                // the current column's full sum plus the best prior prefix-max (`dp3[n]`) wins.
                dp1_next[j] = s2 + dp3[n];

                for k in 0..=j {
                    s2 -= grid[k][i] as i64;
                    let mut v = dp1[k] + s2;
                    if dp3[j] + s2 > v {
                        v = dp3[j] + s2;
                    }
                    if pre + s2 > v {
                        v = pre + s2;
                    }
                    if v > dp1_next[j] {
                        dp1_next[j] = v;
                    }
                    if k == j {
                        dp2_next[j] = v;
                        dp3_next[j] = v;
                        if v > res {
                            res = v;
                        }
                    }
                }

                if i > 0 {
                    let add = grid[j][i] as i64;
                    let a = pre + add;
                    let b = dp2[j] + add;
                    pre = if a > b { a } else { b };
                }
                sum -= grid[j][i] as i64;
            }

            dp2_next[n] = pre;
            dp3_next[n] = pre;
            if pre > res {
                res = pre;
            }
            for j in 1..=n {
                if dp3_next[j - 1] > dp3_next[j] {
                    dp3_next[j] = dp3_next[j - 1];
                }
            }

            std::mem::swap(&mut dp1, &mut dp1_next);
            std::mem::swap(&mut dp2, &mut dp2_next);
            std::mem::swap(&mut dp3, &mut dp3_next);
        }

        res
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
