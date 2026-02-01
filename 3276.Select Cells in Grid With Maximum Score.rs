impl Solution {
    /// Greedy-by-value bitmask DP over rows.
    ///
    /// # Intuition
    /// Each distinct value can be used at most once, so we can process values in any order and
    /// decide which single row (if any) claims that value.
    ///
    /// # Approach
    /// - Flatten the grid into `(value, row)` pairs and sort descending by value.
    /// - Use `dp[mask]` as the best score achievable with chosen rows being a subset of `mask`.
    /// - For each group of equal values, try assigning it to any row not in `mask`, using a
    ///   scratch buffer so the same value is picked at most once.
    /// - The answer is `dp[all_rows]`, which represents the best score over any subset.
    ///
    /// # Complexity
    /// - Time: O(R * C log(R * C) + R * C * 2^R), with R <= 10.
    /// - Space: O(2^R).
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let total_cells: usize = grid.iter().map(Vec::len).sum();

        let mut cells = Vec::with_capacity(total_cells);
        for (row_index, row) in grid.into_iter().enumerate() {
            for value in row {
                cells.push((value, row_index));
            }
        }
        cells.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let states = 1_usize << row_count;
        let mut dp = vec![0_i32; states];
        let mut tmp = vec![0_i32; states];

        let mut index = 0;
        while index < cells.len() {
            let current_value = cells[index].0;
            tmp.copy_from_slice(&dp);

            while index < cells.len() && cells[index].0 == current_value {
                let row_bit = 1_usize << cells[index].1;
                for mask in 0..states {
                    if mask & row_bit == 0 {
                        let new_mask = mask | row_bit;
                        let candidate = dp[mask] + current_value;
                        if candidate > tmp[new_mask] {
                            tmp[new_mask] = candidate;
                        }
                    }
                }
                index += 1;
            }
            std::mem::swap(&mut dp, &mut tmp);
        }

        dp[states - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]];
        assert_eq!(Solution::max_score(grid), 8);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![8, 7, 6], vec![8, 3, 2]];
        assert_eq!(Solution::max_score(grid), 15);
    }

    #[test]
    fn single_cell() {
        let grid = vec![vec![42]];
        assert_eq!(Solution::max_score(grid), 42);
    }

    #[test]
    fn all_duplicates() {
        let grid = vec![vec![5, 5, 5], vec![5], vec![5, 5]];
        assert_eq!(Solution::max_score(grid), 5);
    }

    #[test]
    fn must_skip_one_row() {
        let grid = vec![vec![5], vec![5], vec![5, 6]];
        assert_eq!(Solution::max_score(grid), 11);
    }
}
