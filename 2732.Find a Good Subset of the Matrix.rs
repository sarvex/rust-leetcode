impl Solution {
    /// Find at most two rows whose column-wise AND is all zeros.
    ///
    /// # Intuition
    /// Each row can be represented as a bitmask. An all-zero row is immediately
    /// valid. Otherwise, two rows whose bitmasks AND to zero form a good subset.
    /// With at most 5 columns, there are at most 32 distinct masks, so we can
    /// scan all mask pairs.
    ///
    /// # Approach
    /// 1. Compute the bitmask for each row; if any mask is 0, return that row.
    /// 2. Store first occurrence of each mask in a fixed-size array.
    /// 3. Check all pairs of stored masks for AND == 0.
    ///
    /// # Complexity
    /// - Time: O(m·n + 4^n) where m = rows, n = columns (n ≤ 5)
    /// - Space: O(2^n)
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let columns = grid[0].len();
        let max_mask = 1usize << columns;
        let mut mask_to_row: Vec<Option<i32>> = vec![None; max_mask];

        for (i, row) in grid.iter().enumerate() {
            let mask = row
                .iter()
                .enumerate()
                .fold(0usize, |acc, (j, x)| acc | ((*x as usize) << j));
            if mask == 0 {
                return vec![i as i32];
            }
            if mask_to_row[mask].is_none() {
                mask_to_row[mask] = Some(i as i32);
            }
        }

        for a in 0..max_mask {
            let Some(i) = mask_to_row[a] else {
                continue;
            };
            for b in 0..max_mask {
                let Some(j) = mask_to_row[b] else {
                    continue;
                };
                if (a & b) == 0 {
                    return if i <= j { vec![i, j] } else { vec![j, i] };
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_zero_row() {
        let grid = vec![vec![0, 0], vec![1, 1]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![0]);
    }

    #[test]
    fn two_complementary_rows() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![0, 1]);
    }

    #[test]
    fn example_one() {
        let grid = vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![0, 1]);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![0]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![0]);
    }

    #[test]
    fn no_good_subset() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![]);
    }
}
