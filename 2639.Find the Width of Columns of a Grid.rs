impl Solution {
    /// Find the display width of each column in a grid.
    ///
    /// # Intuition
    /// The width of a column is the maximum string length of its elements
    /// (including the minus sign for negatives).
    ///
    /// # Approach
    /// 1. For each column, compute the string length of every element
    /// 2. Track the maximum per column
    ///
    /// # Complexity
    /// - Time: O(m * n * d) where d is the average digit count
    /// - Space: O(n) for output
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let cols = grid[0].len();
        (0..cols)
            .map(|j| {
                grid.iter()
                    .map(|row| row[j].to_string().len() as i32)
                    .max()
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_signs() {
        assert_eq!(
            Solution::find_column_width(vec![vec![1], vec![-1]]),
            vec![2]
        );
    }

    #[test]
    fn test_multi_column() {
        assert_eq!(
            Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2],]),
            vec![3, 1, 2]
        );
    }
}
