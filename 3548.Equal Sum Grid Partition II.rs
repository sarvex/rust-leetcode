use std::collections::HashMap;

impl Solution {
    /// Partition grid with equal sums using single-cut approach.
    ///
    /// # Intuition
    /// One horizontal or vertical cut splits the grid into two halves. The sums
    /// must be equal, or we may remove one cell (setting its value to zero) if
    /// the half it belongs to stays connected. A cell is safe to remove when the
    /// section is multi-row *and* multi-column (any cell works) or when it sits
    /// at an endpoint of a single-row / single-column section.
    ///
    /// # Approach
    /// 1. Precompute, for every distinct value, the first (min) and last (max)
    ///    row and column where it appears. This answers "does value `v` exist in
    ///    rows `0..=i`?" in O(1) via `first_row[v] <= i`.
    /// 2. Sweep horizontal cuts: accumulate row sums; if the halves differ by
    ///    `d`, look for a removable cell with value `d` in the larger half.
    ///    - Multi-row, multi-col half → any cell works → O(1) lookup.
    ///    - Single-row or single-col half → only the two endpoint cells preserve
    ///      connectivity → check them directly.
    /// 3. Repeat symmetrically for vertical cuts.
    ///
    /// # Complexity
    /// - Time: O(m × n) — one pass to build lookups, one linear sweep per direction
    /// - Space: O(k) where k = number of distinct values
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let total: i64 = grid
            .iter()
            .flat_map(|row| row.iter())
            .map(|&x| x as i64)
            .sum();

        // For every value record the first (min) and last (max) row / column.
        let cap = m * n;
        let mut first_row = HashMap::with_capacity(cap);
        let mut last_row = HashMap::with_capacity(cap);
        let mut first_col = HashMap::with_capacity(cap);
        let mut last_col = HashMap::with_capacity(cap);

        let mut row_sums = Vec::with_capacity(m);
        let mut col_sums = vec![0i64; n];

        for (i, row) in grid.iter().enumerate() {
            let mut rs = 0i64;
            for (j, &val) in row.iter().enumerate() {
                rs += val as i64;
                col_sums[j] += val as i64;
                first_row.entry(val).or_insert(i);
                last_row
                    .entry(val)
                    .and_modify(|r: &mut usize| *r = (*r).max(i))
                    .or_insert(i);
                first_col
                    .entry(val)
                    .and_modify(|c: &mut usize| *c = (*c).min(j))
                    .or_insert(j);
                last_col
                    .entry(val)
                    .and_modify(|c: &mut usize| *c = (*c).max(j))
                    .or_insert(j);
            }
            row_sums.push(rs);
        }

        // --- Horizontal cuts ---
        if m > 1 {
            let mut top = 0i64;
            for i in 0..m - 1 {
                top += row_sums[i];
                let bottom = total - top;
                if top == bottom {
                    return true;
                }
                let diff = top - bottom;
                let target = diff.abs();
                if target > i32::MAX as i64 {
                    continue;
                }
                let tv = target as i32;
                if diff > 0 {
                    if Self::check_h(&grid, n, tv, 0, i, &first_row) {
                        return true;
                    }
                } else if Self::check_h(&grid, n, tv, i + 1, m - 1, &last_row) {
                    return true;
                }
            }
        }

        // --- Vertical cuts ---
        if n > 1 {
            let mut left = 0i64;
            for j in 0..n - 1 {
                left += col_sums[j];
                let right = total - left;
                if left == right {
                    return true;
                }
                let diff = left - right;
                let target = diff.abs();
                if target > i32::MAX as i64 {
                    continue;
                }
                let tv = target as i32;
                if diff > 0 {
                    if Self::check_v(&grid, m, tv, 0, j, &first_col) {
                        return true;
                    }
                } else if Self::check_v(&grid, m, tv, j + 1, n - 1, &last_col) {
                    return true;
                }
            }
        }

        false
    }

    /// Horizontal-cut discount check.
    ///
    /// `lookup` is `first_row` for a top section or `last_row` for a bottom
    /// section. For a top section `first_row[v] <= row_end` proves existence;
    /// for a bottom section `last_row[v] >= row_start` proves existence.
    fn check_h(
        grid: &[Vec<i32>],
        n: usize,
        target: i32,
        rs: usize,
        re: usize,
        lookup: &HashMap<i32, usize>,
    ) -> bool {
        let rows = re - rs + 1;
        if rows == 1 && n == 1 {
            return false;
        }
        if rows == 1 {
            return grid[rs][0] == target || grid[rs][n - 1] == target;
        }
        if n == 1 {
            return grid[rs][0] == target || grid[re][0] == target;
        }
        lookup.get(&target).map_or(false, |&r| r >= rs && r <= re)
    }

    /// Vertical-cut discount check.
    ///
    /// `lookup` is `first_col` for a left section or `last_col` for a right
    /// section.
    fn check_v(
        grid: &[Vec<i32>],
        m: usize,
        target: i32,
        cs: usize,
        ce: usize,
        lookup: &HashMap<i32, usize>,
    ) -> bool {
        let cols = ce - cs + 1;
        if m == 1 && cols == 1 {
            return false;
        }
        if m == 1 {
            return grid[0][cs] == target || grid[0][ce] == target;
        }
        if cols == 1 {
            return grid[0][cs] == target || grid[m - 1][cs] == target;
        }
        lookup.get(&target).map_or(false, |&c| c >= cs && c <= ce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_equal_horizontal_cut() {
        let grid = vec![vec![1, 4], vec![2, 3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn example_2_vertical_cut_with_discount() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn example_3_discount_breaks_connectivity() {
        let grid = vec![vec![1, 2, 4], vec![2, 3, 5]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn example_4_no_valid_cut() {
        let grid = vec![vec![4, 1, 8], vec![3, 2, 6]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_row_equal_split() {
        let grid = vec![vec![2, 1, 3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_row_no_valid_cut() {
        let grid = vec![vec![1, 3, 2]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_column_equal_split() {
        let grid = vec![vec![2], vec![1], vec![3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_column_no_valid_cut() {
        let grid = vec![vec![1], vec![3], vec![2]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn two_by_two_equal() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn large_values() {
        let grid = vec![vec![100000, 100000], vec![100000, 100000]];
        assert!(Solution::can_partition_grid(grid));
    }
}
