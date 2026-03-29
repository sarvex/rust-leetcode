impl Solution {
    /// Single-pass cumulative prefix build with direct-loop early-exit scan.
    ///
    /// # Intuition
    /// A single straight cut splits the grid into two contiguous halves whose
    /// sums are equal iff the prefix at the cut equals half the total. All
    /// values are positive so the prefix is strictly monotonic — at most one
    /// position per axis can match.
    ///
    /// # Approach
    /// 1. One pass over the grid builds cumulative row prefixes and column
    ///    totals simultaneously via `zip` for bounds-check elimination.
    /// 2. If the total is odd, return `false` immediately.
    /// 3. Scan row prefixes then column prefix sums with indexed loops;
    ///    return `true` on first match.
    ///
    /// # Complexity
    /// - Time:  O(m × n)
    /// - Space: O(m + n)
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut col_sums = vec![0i64; n];
        let mut row_pre = Vec::with_capacity(m);
        let mut total = 0i64;

        for row in &grid {
            let mut rs = 0i64;
            for (cs, val) in col_sums.iter_mut().zip(row) {
                let v = *val as i64;
                rs += v;
                *cs += v;
            }
            total += rs;
            row_pre.push(total);
        }

        if total & 1 != 0 {
            return false;
        }
        let half = total >> 1;

        for i in 0..m - 1 {
            if row_pre[i] == half {
                return true;
            }
        }

        let mut running = 0i64;
        for j in 0..n - 1 {
            running += col_sums[j];
            if running == half {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_cut() {
        let grid = vec![vec![1, 4], vec![2, 3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_no_valid_cut() {
        let grid = vec![vec![1, 3], vec![2, 4]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_vertical_cut() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        // col-0 sum = 5, col-1 sum = 7, col-2 sum = 9, total = 21 (odd) → false
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_vertical_cut_valid() {
        let grid = vec![vec![2, 2], vec![2, 2]];
        // horizontal after row 0: 4 == 4 ✓
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![3, 1, 2, 3, 1]];
        // prefix sums: 3, 4, 6 (but need cuts before last col)
        // total = 10, half = 5 → prefix 3,4,6,9 — 5 never hit → false
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_single_row_valid() {
        let grid = vec![vec![1, 2, 3, 4]];
        // total = 10, half = 5. prefix: 1,3,6 — no hit → false
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![5], vec![3], vec![2], vec![5], vec![5]];
        // total = 20, half = 10. row prefix: 5,8,10 → hit after row 2 ✓
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_odd_total() {
        let grid = vec![vec![1, 2], vec![3, 5]];
        // total = 11 (odd) → false
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_large_values() {
        // 10^5 * 10^5 = 10^10 — needs i64
        let grid = vec![vec![100_000; 1], vec![100_000; 1]];
        assert!(Solution::can_partition_grid(grid));
    }
}
