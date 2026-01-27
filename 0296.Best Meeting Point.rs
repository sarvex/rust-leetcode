impl Solution {
    /// Finds the best meeting point minimizing total Manhattan distance using medians.
    ///
    /// # Intuition
    /// The optimal 1D meeting point minimizing total distance is the median.
    /// Separate the 2D problem into independent row and column medians.
    ///
    /// # Approach
    /// 1. Collect row indices and column indices of all people.
    /// 2. Rows are naturally sorted; sort columns.
    /// 3. Compute total Manhattan distance to the median for each dimension.
    ///
    /// # Complexity
    /// - Time: O(m * n + k log k) where k is the number of people
    /// - Space: O(k)
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    rows.push(i as i32);
                    cols.push(j as i32);
                }
            }
        }
        cols.sort_unstable();
        let row_median = rows[rows.len() / 2];
        let col_median = cols[cols.len() / 2];
        rows.iter().map(|r| (r - row_median).abs()).sum::<i32>()
            + cols.iter().map(|c| (c - col_median).abs()).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_grid() {
        let grid = vec![
            vec![1, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
        ];
        assert_eq!(Solution::min_total_distance(grid), 6);
    }

    #[test]
    fn single_person() {
        assert_eq!(Solution::min_total_distance(vec![vec![1]]), 0);
    }
}
