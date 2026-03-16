use std::collections::BTreeSet;

impl Solution {
    /// Finds the three largest distinct rhombus border sums using diagonal prefix sums.
    ///
    /// # Intuition
    /// A rhombus border in a grid consists of four diagonal segments. Precomputing
    /// prefix sums along both `\` and `/` diagonals lets each border sum be
    /// calculated in constant time, reducing the per-center cost to O(1) per size.
    ///
    /// # Approach
    /// 1. Build 1-indexed prefix sums `d1` (main diagonal `\`) and `d2`
    ///    (anti-diagonal `/`).
    /// 2. Enumerate every cell as center and every valid half-diagonal length `s`.
    /// 3. For `s = 0` the rhombus is a single cell. For `s ≥ 1` compute the four
    ///    side sums with the prefix arrays and subtract the four doubly counted
    ///    corners.
    /// 4. Maintain the three largest distinct values in a `BTreeSet`, trimming
    ///    the smallest whenever the set exceeds three entries.
    ///
    /// # Complexity
    /// - Time:  O(m · n · min(m, n))
    /// - Space: O(m · n)
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        // d1[i][j] = prefix along '\' diagonal (1-indexed, zero-padded)
        // d2[i][j] = prefix along '/' diagonal (1-indexed, zero-padded)
        let mut d1 = vec![vec![0; n + 2]; m + 1];
        let mut d2 = vec![vec![0; n + 2]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                d1[i][j] = grid[i - 1][j - 1] + d1[i - 1][j - 1];
                d2[i][j] = grid[i - 1][j - 1] + d2[i - 1][j + 1];
            }
        }

        let mut top = BTreeSet::new();
        let mut insert = |val: i32, set: &mut BTreeSet<i32>| {
            set.insert(val);
            if set.len() > 3 {
                set.pop_first();
            }
        };

        for r in 0..m {
            for c in 0..n {
                insert(grid[r][c], &mut top);

                let max_s = r.min(m - 1 - r).min(c).min(n - 1 - c);
                for s in 1..=max_s {
                    // Four diagonal-segment sums (each includes both endpoints)
                    let s1 = d1[r + 1][c + s + 1] - d1[r - s][c];
                    let s2 = d2[r + s + 1][c + 1] - d2[r][c + s + 2];
                    let s3 = d1[r + s + 1][c + 1] - d1[r][c - s];
                    let s4 = d2[r + 1][c - s + 1] - d2[r - s][c + 2];

                    let corners = grid[r - s][c] + grid[r][c + s] + grid[r + s][c] + grid[r][c - s];

                    insert(s1 + s2 + s3 + s4 - corners, &mut top);
                }
            }
        }

        top.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let grid = vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 3, 4, 2, 3],
            vec![20, 30, 200, 40, 10],
            vec![1, 5, 5, 4, 1],
            vec![4, 3, 2, 2, 5],
        ];
        assert_eq!(Solution::get_biggest_three(grid), vec![228, 216, 211]);
    }

    #[test]
    fn test_example_two() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::get_biggest_three(grid), vec![20, 9, 8]);
    }

    #[test]
    fn test_example_three() {
        let grid = vec![vec![7, 7, 7]];
        assert_eq!(Solution::get_biggest_three(grid), vec![7]);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![42]];
        assert_eq!(Solution::get_biggest_three(grid), vec![42]);
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        assert_eq!(Solution::get_biggest_three(grid), vec![5, 4, 3]);
    }

    #[test]
    fn test_two_by_two() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::get_biggest_three(grid), vec![4, 3, 2]);
    }

    #[test]
    fn test_fewer_than_three_distinct() {
        let grid = vec![vec![5, 5], vec![5, 5]];
        assert_eq!(Solution::get_biggest_three(grid), vec![5]);
    }
}
