use std::collections::HashMap;

impl Solution {
    /// Find at most two rows whose column-wise AND is all zeros.
    ///
    /// # Intuition
    /// Each row can be represented as a bitmask. An all-zero row is immediately
    /// valid. Otherwise, two rows whose bitmasks AND to zero form a good subset.
    /// With at most 5 columns, there are at most 32 distinct masks.
    ///
    /// # Approach
    /// 1. Compute the bitmask for each row; if any mask is 0, return that row.
    /// 2. Store first occurrence of each mask in a `HashMap`.
    /// 3. Check all pairs of stored masks for AND == 0.
    ///
    /// # Complexity
    /// - Time: O(m·n + 4^n) where m = rows, n = columns (n ≤ 5)
    /// - Space: O(2^n)
    pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mask_to_row: HashMap<i32, i32> = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            let mask = row
                .iter()
                .enumerate()
                .fold(0i32, |acc, (j, &x)| acc | (x << j));
            if mask == 0 {
                return vec![i as i32];
            }
            mask_to_row.entry(mask).or_insert(i as i32);
        }
        for (&a, &i) in &mask_to_row {
            for (&b, &j) in &mask_to_row {
                if (a & b) == 0 {
                    return vec![i.min(j), i.max(j)];
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
        let result = Solution::good_subsetof_binary_matrix(grid);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn no_good_subset() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::good_subsetof_binary_matrix(grid), vec![]);
    }
}
