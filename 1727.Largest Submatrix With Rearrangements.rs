impl Solution {
    /// Greedy column sorting on histogram heights.
    ///
    /// # Intuition
    /// For each row, compute the consecutive-ones height ending at that row for
    /// every column (like a histogram). Since we can rearrange columns freely,
    /// sort the heights in descending order per row. The widest rectangle of
    /// height `h` at sorted position `j` has area `h * (j + 1)`.
    ///
    /// # Approach
    /// 1. Build a height matrix: for each cell, if `matrix[i][j] == 1` then
    ///    `height[i][j] = height[i-1][j] + 1`, else `0`.
    /// 2. For each row, sort its heights in descending order.
    /// 3. Scan the sorted row: at index `j` the rectangle is `height * (j + 1)`.
    ///    Track the global maximum.
    ///
    /// # Complexity
    /// - Time: O(m · n · log n) — one sort per row.
    /// - Space: O(n) — in-place mutation of a single row buffer.
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![0i32; n];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                heights[j] = if matrix[i][j] == 1 { heights[j] + 1 } else { 0 };
            }

            let mut sorted = heights.clone();
            sorted.sort_unstable_by(|a, b| b.cmp(a));

            for (j, &h) in sorted.iter().enumerate() {
                if h == 0 {
                    break;
                }
                result = result.max(h * (j as i32 + 1));
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
        let matrix = vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 4);
    }

    #[test]
    fn test_example_2() {
        let matrix = vec![vec![1, 0, 1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 3);
    }

    #[test]
    fn test_example_3() {
        let matrix = vec![vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 2);
    }

    #[test]
    fn test_single_cell_one() {
        let matrix = vec![vec![1]];
        assert_eq!(Solution::largest_submatrix(matrix), 1);
    }

    #[test]
    fn test_single_cell_zero() {
        let matrix = vec![vec![0]];
        assert_eq!(Solution::largest_submatrix(matrix), 0);
    }

    #[test]
    fn test_all_ones() {
        let matrix = vec![vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(Solution::largest_submatrix(matrix), 6);
    }

    #[test]
    fn test_all_zeros() {
        let matrix = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::largest_submatrix(matrix), 0);
    }
}
