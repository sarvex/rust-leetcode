impl Solution {
    /// Dynamic programming by value groups.
    ///
    /// # Intuition
    /// Any valid move must go to a strictly larger value, so when we process cells
    /// in increasing value order, the best previous step for a cell is already known.
    /// The only candidates are the best paths seen so far in its row or its column.
    ///
    /// # Approach
    /// - Flatten the matrix into `(value, row, col)` tuples and sort by value.
    /// - Track `row_best` and `col_best` as the best path lengths for each row/column
    ///   using only strictly smaller values.
    /// - For each group of equal values, compute `dp = 1 + max(row_best[row], col_best[col])`
    ///   without updating `row_best`/`col_best` until the entire group is processed.
    /// - Apply the group's updates and keep the global maximum.
    ///
    /// # Complexity
    /// - Time: O(m*n log(m*n))
    /// - Space: O(m*n + m + n)
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let row_count = mat.len();
        let col_count = mat.first().map_or(0, |row| row.len());
        if row_count == 0 || col_count == 0 {
            return 0;
        }

        let mut cells = Vec::with_capacity(row_count * col_count);
        for (row, values) in mat.iter().enumerate() {
            for (col, &value) in values.iter().enumerate() {
                cells.push((value, row, col));
            }
        }

        cells.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut row_best = vec![0i32; row_count];
        let mut col_best = vec![0i32; col_count];
        let mut answer = 1;
        let mut index = 0;

        while index < cells.len() {
            let value = cells[index].0;
            let mut next_index = index + 1;
            while next_index < cells.len() && cells[next_index].0 == value {
                next_index += 1;
            }

            let mut updates: Vec<(usize, usize, i32)> = Vec::with_capacity(next_index - index);
            for k in index..next_index {
                let (_, row, col) = cells[k];
                let best_prev = row_best[row].max(col_best[col]);
                let dp = best_prev + 1;
                updates.push((row, col, dp));
                if dp > answer {
                    answer = dp;
                }
            }

            for (row, col, dp) in updates {
                if dp > row_best[row] {
                    row_best[row] = dp;
                }
                if dp > col_best[col] {
                    col_best[col] = dp;
                }
            }

            index = next_index;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        let mat = vec![vec![3, 1], vec![3, 4]];
        assert_eq!(Solution::max_increasing_cells(mat), 2);
    }

    #[test]
    fn test_example2() {
        let mat = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::max_increasing_cells(mat), 1);
    }

    #[test]
    fn test_example3() {
        let mat = vec![vec![3, 1, 6], vec![-9, 5, 7]];
        assert_eq!(Solution::max_increasing_cells(mat), 4);
    }

    #[test]
    fn test_single_cell() {
        let mat = vec![vec![42]];
        assert_eq!(Solution::max_increasing_cells(mat), 1);
    }

    #[test]
    fn test_boundary_values() {
        let mat = vec![vec![-100_000, 0, 100_000]];
        assert_eq!(Solution::max_increasing_cells(mat), 3);
    }
}
