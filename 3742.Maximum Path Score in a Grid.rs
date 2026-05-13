impl Solution {
    /// Dynamic programming over (row, column, cost) with rolling rows.
    ///
    /// # Intuition
    /// Each path from the top-left to the bottom-right accumulates a score
    /// and a cost that depend only on the multiset of visited cells. With
    /// only right and down moves, standard grid DP applies, but the cost
    /// budget `k` must be tracked as an extra dimension. Since only the
    /// previous row is ever consulted, the full table collapses to two rows.
    ///
    /// # Approach
    /// Maintain `prev[j][c]` and `curr[j][c]` — the maximum score reaching
    /// cell `(i, j)` while spending exactly `c` units of cost, using `-1`
    /// as the "unreachable" sentinel. For each cell with per-cell score
    /// `sc` and cost `co` (derived from its grid value), combine the best
    /// of the top and left neighbours at cost `c - co`, then add `sc`.
    /// After filling the final row, scan the destination column over all
    /// budgets `0..=k` and return the maximum; `-1` if none is reachable.
    ///
    /// # Complexity
    /// - Time: O(m · n · k) — each (row, column, cost) triple is visited once.
    /// - Space: O(n · k) — two rolling rows of `k + 1` cost slots each.
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let k = k as usize;

        let contrib = |v: i32| -> (i32, usize) {
            match v {
                1 => (1, 1),
                2 => (2, 1),
                _ => (0, 0),
            }
        };

        let row_len = k + 1;
        let mut prev = vec![-1_i32; n * row_len];
        let mut curr = vec![-1_i32; n * row_len];

        prev[0] = 0;

        for j in 1..n {
            let (sc, co) = contrib(grid[0][j]);
            for c in 0..=k {
                if c >= co {
                    let left = prev[(j - 1) * row_len + (c - co)];
                    if left >= 0 {
                        prev[j * row_len + c] = left + sc;
                    }
                }
            }
        }

        for row in grid.iter().take(m).skip(1) {
            for slot in curr.iter_mut() {
                *slot = -1;
            }

            for j in 0..n {
                let (sc, co) = contrib(row[j]);
                for c in 0..=k {
                    if c < co {
                        continue;
                    }
                    let from_top = prev[j * row_len + (c - co)];
                    let from_left = if j > 0 {
                        curr[(j - 1) * row_len + (c - co)]
                    } else {
                        -1
                    };
                    let best = from_top.max(from_left);
                    if best >= 0 {
                        curr[j * row_len + c] = best + sc;
                    }
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        let dest = (n - 1) * row_len;
        let mut ans = -1_i32;
        for c in 0..=k {
            if prev[dest + c] > ans {
                ans = prev[dest + c];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let grid = vec![vec![0, 1], vec![2, 0]];
        assert_eq!(Solution::max_path_score(grid, 1), 2);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::max_path_score(grid, 1), -1);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::max_path_score(vec![vec![0]], 0), 0);
        assert_eq!(Solution::max_path_score(vec![vec![0]], 5), 0);
    }

    #[test]
    fn single_row_zero_budget() {
        assert_eq!(Solution::max_path_score(vec![vec![0, 0, 0, 0]], 0), 0);
        assert_eq!(Solution::max_path_score(vec![vec![0, 1, 0]], 0), -1);
        assert_eq!(Solution::max_path_score(vec![vec![0, 1, 0]], 1), 1);
        assert_eq!(Solution::max_path_score(vec![vec![0, 2, 0]], 1), 2);
    }

    #[test]
    fn single_column() {
        assert_eq!(
            Solution::max_path_score(vec![vec![0], vec![2], vec![2], vec![0]], 2),
            4
        );
        assert_eq!(
            Solution::max_path_score(vec![vec![0], vec![2], vec![2], vec![0]], 1),
            -1
        );
    }

    #[test]
    fn prefers_twos_over_ones() {
        // Two paths right-then-down vs down-then-right: pick the one with more 2s.
        // grid:
        // 0 1 1
        // 2 2 0
        // Path down then right: 0 -> 2 -> 2 -> 0, cost 2, score 4.
        // Path right-right-down-down would exceed cost budget of 2.
        let grid = vec![vec![0, 1, 1], vec![2, 2, 0]];
        assert_eq!(Solution::max_path_score(grid, 2), 4);
    }

    #[test]
    fn tight_budget_forces_zero_path() {
        // 0 2 1
        // 1 0 2
        // 2 1 0
        // With k = 0 we can only traverse cells of value 0; no all-zero path exists.
        let grid = vec![vec![0, 2, 1], vec![1, 0, 2], vec![2, 1, 0]];
        assert_eq!(Solution::max_path_score(grid, 0), -1);
    }

    #[test]
    fn large_budget_returns_max_possible() {
        // With enough budget, every 2 is worth taking.
        // 0 2 2
        // 2 2 2
        // 2 2 2
        // Path length is m + n - 1 = 5 cells; (0,0)=0, remaining 4 cells = 2s.
        // Best any monotone path from (0,0) to (2,2): 4 twos -> score 8, cost 4.
        let grid = vec![vec![0, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];
        assert_eq!(Solution::max_path_score(grid, 1000), 8);
    }

    #[test]
    fn stress_all_zeros() {
        // 200x200 of zeros: score 0, cost 0.
        let grid = vec![vec![0; 200]; 200];
        assert_eq!(Solution::max_path_score(grid, 1000), 0);
    }

    #[test]
    fn stress_all_twos_except_start() {
        // 200x200 with 2s everywhere except (0,0) = 0.
        // Path length = 399; 398 twos -> cost 398, score 796.
        let mut grid = vec![vec![2; 200]; 200];
        grid[0][0] = 0;
        assert_eq!(Solution::max_path_score(grid.clone(), 1000), 796);
        // Budget exactly enough.
        assert_eq!(Solution::max_path_score(grid.clone(), 398), 796);
        // Budget one short.
        assert_eq!(Solution::max_path_score(grid, 397), -1);
    }
}
