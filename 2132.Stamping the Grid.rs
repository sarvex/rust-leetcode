impl Solution {
    /// Checks if all empty cells can be covered by non-overlapping stamps using 2D prefix sums.
    ///
    /// # Intuition
    /// We need to determine whether every empty cell (0) in the grid can be
    /// covered by at least one stamp placement. Using 2D prefix sums, we can
    /// quickly check if a stamp-sized region is entirely empty. A 2D difference
    /// array then marks all stampable regions, and a final prefix sum verifies
    /// coverage of every empty cell.
    ///
    /// # Approach
    /// 1. Build a 2D prefix sum of the grid to query subregion sums in O(1).
    /// 2. For each empty cell, check if a stamp anchored there fits within
    ///    bounds and covers only empty cells (region sum is zero).
    /// 3. Mark valid stamp placements in a 2D difference array.
    /// 4. Compute the prefix sum of the difference array and verify every
    ///    empty cell has a non-zero coverage count.
    ///
    /// # Complexity
    /// - Time: O(n * m)
    /// - Space: O(n * m)
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        let sh = stamp_height as usize;
        let sw = stamp_width as usize;

        let mut prefix = vec![vec![0i32; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                prefix[i + 1][j + 1] =
                    prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + grid[i][j];
            }
        }

        let region_sum = |r1: usize, c1: usize, r2: usize, c2: usize| -> i32 {
            prefix[r2][c2] - prefix[r2][c1] - prefix[r1][c2] + prefix[r1][c1]
        };

        let mut diff = vec![vec![0i32; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    continue;
                }
                let r2 = i + sh;
                let c2 = j + sw;
                if r2 <= n && c2 <= m && region_sum(i, j, r2, c2) == 0 {
                    diff[i][j] += 1;
                    diff[r2][c2] += 1;
                    diff[r2][j] -= 1;
                    diff[i][c2] -= 1;
                }
            }
        }

        let mut coverage = vec![vec![0i32; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                coverage[i + 1][j + 1] =
                    coverage[i][j + 1] + coverage[i + 1][j] - coverage[i][j] + diff[i][j];
                if grid[i][j] == 0 && coverage[i + 1][j + 1] == 0 {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stampable_grid() {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        assert!(Solution::possible_to_stamp(grid, 4, 3));
    }

    #[test]
    fn test_unstampable_grid() {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0]];
        assert!(!Solution::possible_to_stamp(grid, 2, 2));
    }

    #[test]
    fn test_all_occupied() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert!(Solution::possible_to_stamp(grid, 1, 1));
    }
}
