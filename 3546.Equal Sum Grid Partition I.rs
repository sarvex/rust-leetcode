impl Solution {
    /// Zero-allocation prefix scan for horizontal and vertical grid cuts.
    ///
    /// # Intuition
    /// A straight cut divides the grid into two halves with equal sums iff
    /// the prefix at the cut equals exactly half the total.  All values are
    /// positive so the prefix is strictly monotonic — at most one position
    /// per axis can match.
    ///
    /// # Approach
    /// 1. Compute the total via a single `flatten` pass.
    /// 2. If the total is odd, no equal partition exists.
    /// 3. Accumulate row-wise prefix sums, returning on the first
    ///    horizontal cut that equals half.
    /// 4. Accumulate column-wise prefix sums, returning on the first
    ///    vertical cut that equals half.
    ///
    /// No auxiliary vectors are allocated — only scalar accumulators.
    ///
    /// # Complexity
    /// - Time:  O(m × n)
    /// - Space: O(1)
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        let total: i64 = grid.iter().flatten().map(|v| *v as i64).sum();

        if total & 1 != 0 {
            return false;
        }
        let half = total >> 1;

        let mut prefix = 0i64;
        for row in grid.iter().take(m - 1) {
            prefix += row.iter().map(|v| *v as i64).sum::<i64>();
            if prefix == half {
                return true;
            }
        }

        let mut prefix = 0i64;
        for j in 0..n - 1 {
            prefix += grid.iter().map(|r| r[j] as i64).sum::<i64>();
            if prefix == half {
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
