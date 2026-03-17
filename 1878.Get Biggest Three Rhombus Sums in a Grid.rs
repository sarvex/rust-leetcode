impl Solution {
    /// Finds the three largest distinct rhombus border sums using diagonal prefix sums.
    ///
    /// # Intuition
    /// A rhombus border in a grid consists of four diagonal segments. Precomputing
    /// prefix sums along both `\` and `/` diagonals lets each border sum be
    /// calculated in constant time, reducing the per-centre cost to O(1) per size.
    ///
    /// # Approach
    /// 1. Flatten the grid and build flat, 1-indexed prefix arrays `d1` (main
    ///    diagonal `\`) and `d2` (anti-diagonal `/`) for cache-friendly access.
    /// 2. Enumerate every cell as centre and every valid half-diagonal length `s`.
    /// 3. For `s = 0` the rhombus is a single cell. For `s ≥ 1` compute the four
    ///    side sums via the prefix arrays and subtract the four doubly counted
    ///    corners.
    /// 4. Maintain the three largest distinct values in a fixed `[i32; 3]` array,
    ///    avoiding all heap allocation in the hot loop.
    ///
    /// # Complexity
    /// - Time:  O(m · n · min(m, n))
    /// - Space: O(m · n)
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let g: Vec<i32> = grid.into_iter().flatten().collect();
        let w = n + 2;
        let mut d1 = vec![0i32; (m + 1) * w];
        let mut d2 = vec![0i32; (m + 1) * w];

        for i in 1..=m {
            for j in 1..=n {
                let v = g[(i - 1) * n + (j - 1)];
                d1[i * w + j] = v + d1[(i - 1) * w + (j - 1)];
                d2[i * w + j] = v + d2[(i - 1) * w + (j + 1)];
            }
        }

        let mut top = [0i32; 3];
        let mut tl = 0usize;

        for r in 0..m {
            for c in 0..n {
                Self::push(&mut top, &mut tl, g[r * n + c]);

                let max_s = r.min(m - 1 - r).min(c).min(n - 1 - c);
                for s in 1..=max_s {
                    let s1 = d1[(r + 1) * w + c + s + 1] - d1[(r - s) * w + c];
                    let s2 = d2[(r + s + 1) * w + c + 1] - d2[r * w + c + s + 2];
                    let s3 = d1[(r + s + 1) * w + c + 1] - d1[r * w + (c - s)];
                    let s4 = d2[(r + 1) * w + (c - s) + 1] - d2[(r - s) * w + c + 2];
                    let corners = g[(r - s) * n + c]
                        + g[r * n + c + s]
                        + g[(r + s) * n + c]
                        + g[r * n + (c - s)];

                    Self::push(&mut top, &mut tl, s1 + s2 + s3 + s4 - corners);
                }
            }
        }

        let mut result = top[..tl].to_vec();
        result.sort_unstable_by(|a, b| b.cmp(a));
        result
    }

    #[inline]
    fn push(top: &mut [i32; 3], len: &mut usize, val: i32) {
        for i in 0..*len {
            if top[i] == val {
                return;
            }
        }
        if *len < 3 {
            top[*len] = val;
            *len += 1;
            top[..*len].sort_unstable();
        } else if val > top[0] {
            top[0] = val;
            top.sort_unstable();
        }
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
