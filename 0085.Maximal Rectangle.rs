impl Solution {
    /// Row-by-row histogram reduction for maximal rectangle in a binary matrix.
    ///
    /// # Intuition
    /// Each row of the matrix defines a histogram where bar heights are the
    /// count of consecutive '1's stacked vertically. The maximal rectangle
    /// is the largest rectangle in any of these histograms.
    ///
    /// # Approach
    /// Maintain a heights array updated row by row: increment for '1',
    /// reset to 0 for '0'. For each row, compute the largest rectangle
    /// in the histogram using a monotonic stack with a sentinel element.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each row processed in O(n)
    /// - Space: O(n) — heights array and stack
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let cols = matrix[0].len();
        let mut heights = vec![0i32; cols + 1];
        let mut stack = Vec::with_capacity(cols + 1);
        let mut max_area = 0;

        for row in &matrix {
            for (i, &cell) in row.iter().enumerate() {
                heights[i] = if cell == '1' { heights[i] + 1 } else { 0 };
            }

            stack.clear();
            for (i, &h) in heights.iter().enumerate() {
                while let Some(&top) = stack.last() {
                    if heights[top] > h {
                        stack.pop();
                        let height = heights[top];
                        let width = match stack.last() {
                            Some(&left) => i - left - 1,
                            None => i,
                        };
                        max_area = max_area.max(height * width as i32);
                    } else {
                        break;
                    }
                }
                stack.push(i);
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }

    #[test]
    fn empty_matrix() {
        assert_eq!(Solution::maximal_rectangle(vec![]), 0);
    }

    #[test]
    fn single_zero() {
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
    }

    #[test]
    fn single_one() {
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
    }

    #[test]
    fn all_ones() {
        let matrix = vec![vec!['1', '1'], vec!['1', '1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 4);
    }

    #[test]
    fn single_row() {
        let matrix = vec![vec!['1', '1', '1', '1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 4);
    }

    #[test]
    fn single_column() {
        let matrix = vec![vec!['1'], vec!['1'], vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 3);
    }
}
