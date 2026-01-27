impl Solution {
    /// DP with first and second minimum tracking per row.
    ///
    /// # Intuition
    /// Each cell in the falling path must come from a different column in the
    /// previous row. By tracking only the two smallest values and their column
    /// indices from the previous row, we avoid the O(n) inner scan per cell.
    ///
    /// # Approach
    /// 1. Initialize DP from the first row
    /// 2. For each subsequent row, find the minimum and second minimum of the
    ///    previous DP row
    /// 3. For each column, add the minimum (or second minimum if same column)
    /// 4. Return the overall minimum of the final row
    ///
    /// # Complexity
    /// - Time: O(n²) — two passes per row
    /// - Space: O(n) single DP array
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![0i32; n];

        for row in &grid {
            let (mut min1, mut min1_idx, mut min2) = (i32::MAX, 0, i32::MAX);
            for (j, &val) in dp.iter().enumerate() {
                if val < min1 {
                    min2 = min1;
                    min1 = val;
                    min1_idx = j;
                } else if val < min2 {
                    min2 = val;
                }
            }

            let mut new_dp = Vec::with_capacity(n);
            for (j, &val) in row.iter().enumerate() {
                let prev = if j == min1_idx { min2 } else { min1 };
                new_dp.push(val + if prev == i32::MAX { 0 } else { prev });
            }
            dp = new_dp;
        }

        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
            13
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_falling_path_sum(vec![vec![7]]), 7);
    }

    #[test]
    fn two_by_two() {
        assert_eq!(
            Solution::min_falling_path_sum(vec![vec![1, 2], vec![3, 4]]),
            5
        );
    }
}
