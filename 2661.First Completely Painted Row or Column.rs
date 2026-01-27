use std::collections::HashMap;

impl Solution {
    /// Find the earliest index in arr that completes a row or column.
    ///
    /// # Intuition
    /// Map each value to its (row, col) position. Track fill counts per row
    /// and column, returning the first index where any reaches its target.
    ///
    /// # Approach
    /// 1. Build a value-to-position map from the matrix
    /// 2. Iterate through arr, incrementing row/col counters
    /// 3. Return the first index where a row or column is fully painted
    ///
    /// # Complexity
    /// - Time: O(m * n)
    /// - Space: O(m * n)
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let idx: HashMap<i32, (usize, usize)> = mat
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &val)| (val, (i, j))))
            .collect();

        let mut row_count = vec![0usize; m];
        let mut col_count = vec![0usize; n];

        for (k, &val) in arr.iter().enumerate() {
            let &(i, j) = idx.get(&val).unwrap();
            row_count[i] += 1;
            col_count[j] += 1;
            if row_count[i] == n || col_count[j] == m {
                return k as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]],),
            2
        );
    }

    #[test]
    fn test_column_first() {
        assert_eq!(
            Solution::first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]],
            ),
            3
        );
    }
}
