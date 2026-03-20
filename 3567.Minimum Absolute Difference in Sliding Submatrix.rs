impl Solution {
    /// Sorts each k×k submatrix window and scans for the minimum consecutive gap.
    ///
    /// # Intuition
    /// The minimum absolute difference between distinct values equals the smallest
    /// gap between consecutive elements when sorted. A single reused buffer with
    /// bulk row copies and early termination minimizes overhead.
    ///
    /// # Approach
    /// 1. Short-circuit when k = 1: every 1×1 window has one element, answer is all zeros.
    /// 2. Preallocate one buffer of capacity k² and the full result matrix.
    /// 3. For each window position, clear the buffer and bulk-copy each row slice
    ///    via `extend_from_slice` (single `memcpy` per row).
    /// 4. Sort in-place with `sort_unstable` (no allocation, pattern-defeating quicksort).
    /// 5. Scan consecutive pairs; exit immediately when gap equals 1.
    ///
    /// # Complexity
    /// - Time: O((m − k + 1)(n − k + 1) · k² log k²), O(1) when k = 1
    /// - Space: O(k²) reused buffer + O(output)
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let rows = m + 1 - k;
        let cols = n + 1 - k;
        let mut result = vec![vec![0i32; cols]; rows];

        if k == 1 {
            return result;
        }

        let mut vals = Vec::with_capacity(k * k);

        for i in 0..rows {
            for j in 0..cols {
                vals.clear();
                for r in i..i + k {
                    vals.extend_from_slice(&grid[r][j..j + k]);
                }
                vals.sort_unstable();

                let mut best = i32::MAX;
                for w in 1..vals.len() {
                    let d = vals[w] - vals[w - 1];
                    if d > 0 && d < best {
                        best = d;
                        if best == 1 {
                            break;
                        }
                    }
                }
                result[i][j] = if best == i32::MAX { 0 } else { best };
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![1, 8], vec![3, -2]];
        assert_eq!(Solution::min_abs_diff(grid, 2), vec![vec![2]]);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![3, -1]];
        assert_eq!(Solution::min_abs_diff(grid, 1), vec![vec![0, 0]]);
    }

    #[test]
    fn test_example_3() {
        let grid = vec![vec![1, -2, 3], vec![2, 3, 5]];
        assert_eq!(Solution::min_abs_diff(grid, 2), vec![vec![1, 2]]);
    }

    #[test]
    fn test_single_element() {
        let grid = vec![vec![42]];
        assert_eq!(Solution::min_abs_diff(grid, 1), vec![vec![0]]);
    }

    #[test]
    fn test_all_same_values() {
        let grid = vec![vec![5, 5], vec![5, 5]];
        assert_eq!(Solution::min_abs_diff(grid, 2), vec![vec![0]]);
    }

    #[test]
    fn test_k_equals_1() {
        let grid = vec![vec![10, 20], vec![30, 40]];
        assert_eq!(
            Solution::min_abs_diff(grid, 1),
            vec![vec![0, 0], vec![0, 0]]
        );
    }

    #[test]
    fn test_negative_values() {
        let grid = vec![vec![-5, -3], vec![-1, 0]];
        assert_eq!(Solution::min_abs_diff(grid, 2), vec![vec![1]]);
    }

    #[test]
    fn test_large_grid_k_equals_full() {
        let grid = vec![vec![100, 200, 300], vec![150, 250, 350], vec![50, 175, 400]];
        assert_eq!(Solution::min_abs_diff(grid, 3), vec![vec![25]]);
    }

    #[test]
    fn test_early_termination() {
        let grid = vec![vec![1, 2], vec![100, 200]];
        assert_eq!(Solution::min_abs_diff(grid, 2), vec![vec![1]]);
    }
}
