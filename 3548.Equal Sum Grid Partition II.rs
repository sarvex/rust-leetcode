use std::collections::HashMap;

impl Solution {
    /// Partition grid with equal sums using single-cut approach.
    ///
    /// # Intuition
    /// We can make one horizontal or vertical cut to split the grid into two sections.
    /// The sums must be equal, or we can discount one cell (remove from sum) if the
    /// remaining section stays connected. A cell can only be discounted from corners
    /// of single-row/column sections; otherwise connectivity is broken.
    ///
    /// # Approach
    /// 1. Try each horizontal cut position, maintaining top/bottom value frequency maps
    /// 2. For each cut, check if sums are equal or can be equalized by discounting
    /// 3. For single-row/column sections, only corners can be discounted
    /// 4. For multi-row multi-column sections, any cell can be discounted
    /// 5. Repeat for vertical cuts
    ///
    /// # Complexity
    /// - Time: O(m * n) - iterate through all cells for both cut directions
    /// - Space: O(m * n) - frequency maps store all unique values
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let total: i64 = grid
            .iter()
            .flat_map(|row| row.iter())
            .map(|&x| x as i64)
            .sum();

        if m > 1 && Self::try_horizontal_cuts(&grid, m, n, total) {
            return true;
        }

        if n > 1 && Self::try_vertical_cuts(&grid, m, n, total) {
            return true;
        }

        false
    }

    fn try_horizontal_cuts(grid: &[Vec<i32>], m: usize, n: usize, total: i64) -> bool {
        let mut top_count: HashMap<i32, usize> = HashMap::new();
        let mut bottom_count: HashMap<i32, usize> =
            grid.iter()
                .flat_map(|row| row.iter())
                .fold(HashMap::new(), |mut acc, &v| {
                    *acc.entry(v).or_insert(0) += 1;
                    acc
                });

        let mut top_sum: i64 = 0;

        for i in 0..m - 1 {
            for j in 0..n {
                let val = grid[i][j];
                top_sum += val as i64;
                *top_count.entry(val).or_insert(0) += 1;
                if let Some(cnt) = bottom_count.get_mut(&val) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        bottom_count.remove(&val);
                    }
                }
            }

            let bottom_sum = total - top_sum;

            if top_sum == bottom_sum {
                return true;
            }

            let diff = (top_sum - bottom_sum).abs();

            if top_sum > bottom_sum {
                if Self::can_remove(grid, &top_count, 0, i, 0, n - 1, diff) {
                    return true;
                }
            } else if Self::can_remove(grid, &bottom_count, i + 1, m - 1, 0, n - 1, diff) {
                return true;
            }
        }

        false
    }

    fn try_vertical_cuts(grid: &[Vec<i32>], m: usize, n: usize, total: i64) -> bool {
        let mut left_count: HashMap<i32, usize> = HashMap::new();
        let mut right_count: HashMap<i32, usize> =
            grid.iter()
                .flat_map(|row| row.iter())
                .fold(HashMap::new(), |mut acc, &v| {
                    *acc.entry(v).or_insert(0) += 1;
                    acc
                });

        let mut left_sum: i64 = 0;

        for j in 0..n - 1 {
            for i in 0..m {
                let val = grid[i][j];
                left_sum += val as i64;
                *left_count.entry(val).or_insert(0) += 1;
                if let Some(cnt) = right_count.get_mut(&val) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        right_count.remove(&val);
                    }
                }
            }

            let right_sum = total - left_sum;

            if left_sum == right_sum {
                return true;
            }

            let diff = (left_sum - right_sum).abs();

            if left_sum > right_sum {
                if Self::can_remove(grid, &left_count, 0, m - 1, 0, j, diff) {
                    return true;
                }
            } else if Self::can_remove(grid, &right_count, 0, m - 1, j + 1, n - 1, diff) {
                return true;
            }
        }

        false
    }

    fn can_remove(
        grid: &[Vec<i32>],
        section_count: &HashMap<i32, usize>,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
        target: i64,
    ) -> bool {
        if target > i32::MAX as i64 || target <= 0 {
            return false;
        }
        let target = target as i32;

        let num_rows = row_end - row_start + 1;
        let num_cols = col_end - col_start + 1;

        if num_rows == 1 && num_cols == 1 {
            return false;
        }

        if num_rows == 1 {
            return grid[row_start][col_start] == target || grid[row_start][col_end] == target;
        }

        if num_cols == 1 {
            return grid[row_start][col_start] == target || grid[row_end][col_start] == target;
        }

        section_count.get(&target).map_or(false, |&c| c > 0)
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
        // After col 1: left=[2,1]=3, right=[3]=3. Equal!
        let grid = vec![vec![2, 1, 3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_row_no_valid_cut() {
        // After col 0: left=1, right=5, diff=4. Corners 3,2. No match.
        // After col 1: left=4, right=2, diff=2. Corners 1,3. No match.
        let grid = vec![vec![1, 3, 2]];
        assert!(!Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_column_equal_split() {
        // After row 1: top=[2,1]=3, bottom=[3]=3. Equal!
        let grid = vec![vec![2], vec![1], vec![3]];
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn single_column_no_valid_cut() {
        // After row 0: top=1, bottom=5, diff=4. Corners 3,2. No match.
        // After row 1: top=4, bottom=2, diff=2. Corners 1,3. No match.
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
