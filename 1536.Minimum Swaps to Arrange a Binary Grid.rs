impl Solution {
    /// Greedy row bubbling based on each row's trailing zeros.
    ///
    /// # Intuition
    /// A grid is valid when for every row `i`, all columns `j > i` are `0`.
    /// That means the rightmost `1` in row `i` must be at position `<= i`, i.e.
    /// the row must have at least `n - 1 - i` trailing zeros.
    ///
    /// Since the only operation is swapping adjacent rows, when building the grid
    /// from top to bottom we should pick the earliest row that satisfies the needed
    /// trailing zeros and bubble it up; this is optimal because any solution must
    /// move some satisfying row into position `i`, and choosing the closest one
    /// minimizes swaps at that step without harming future feasibility.
    ///
    /// # Approach
    /// - Precompute `trailing_zeros[r]` for each row `r`.
    /// - For each target row index `i` from `0..n`:
    ///   - Need `need = n - 1 - i` trailing zeros.
    ///   - Find the smallest `j >= i` with `trailing_zeros[j] >= need`.
    ///   - If none exists, return `-1`.
    ///   - Bubble row `j` up to `i` via adjacent swaps; add `j - i` to the answer.
    ///
    /// # Complexity
    /// - Time: \(O(n^2)\) (at most \(n\) searches and \(n\) bubbles, `n <= 200`)
    /// - Space: \(O(n)\)
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut trailing_zeros: Vec<usize> = grid
            .iter()
            .map(|row| {
                row.iter()
                    .rposition(|&v| v == 1)
                    .map_or(n, |pos| n - 1 - pos)
            })
            .collect();

        let mut swaps: i32 = 0;
        for i in 0..n {
            let need = n - 1 - i;
            let Some(mut j) = (i..n).find(|&r| trailing_zeros[r] >= need) else {
                return -1;
            };

            while j > i {
                trailing_zeros.swap(j, j - 1);
                swaps += 1;
                j -= 1;
            }
        }
        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]];
        assert_eq!(Solution::min_swaps(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
        ];
        assert_eq!(Solution::min_swaps(grid), -1);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]];
        assert_eq!(Solution::min_swaps(grid), 0);
    }

    #[test]
    fn test_n_equals_1_always_valid() {
        assert_eq!(Solution::min_swaps(vec![vec![0]]), 0);
        assert_eq!(Solution::min_swaps(vec![vec![1]]), 0);
    }

    #[test]
    fn test_impossible_small() {
        // Row0 needs at least 1 trailing zero, but both rows end with 1.
        let grid = vec![vec![1, 1], vec![0, 1]];
        assert_eq!(Solution::min_swaps(grid), -1);
    }
}

