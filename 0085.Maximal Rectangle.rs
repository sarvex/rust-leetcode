/// Finds the maximal rectangle containing only '1's in a binary matrix.
///
/// # Intuition
/// Reduce to largest rectangle in histogram problem. Each row forms a histogram
/// where bar heights are consecutive '1's stacked vertically.
///
/// # Approach
/// 1. Maintain heights array updated row by row
/// 2. Use single-pass monotonic stack with sentinel to compute max rectangle
/// 3. When popping, compute area immediately: height * (right - left - 1)
/// 4. Reuse stack allocation across rows for efficiency
///
/// # Complexity
/// - Time: O(m * n) where m is rows and n is columns
/// - Space: O(n) for heights and stack arrays
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let cols = matrix[0].len();
        let mut heights = vec![0_i32; cols + 1]; // Extra sentinel at end
        let mut stack = Vec::with_capacity(cols + 1);
        let mut max_area = 0;

        for row in &matrix {
            // Update heights in-place
            for (i, &cell) in row.iter().enumerate() {
                heights[i] = if cell == '1' { heights[i] + 1 } else { 0 };
            }
            // heights[cols] stays 0 as sentinel

            max_area = max_area.max(Self::largest_rectangle(&heights, &mut stack));
        }

        max_area
    }

    /// Single-pass monotonic stack algorithm with sentinel.
    /// Computes area on-the-fly when popping elements.
    #[inline]
    fn largest_rectangle(heights: &[i32], stack: &mut Vec<usize>) -> i32 {
        stack.clear();
        let mut max_area = 0;

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

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rectangle() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 6);
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<char>> = vec![];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }

    #[test]
    fn test_single_zero() {
        let matrix = vec![vec!['0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }

    #[test]
    fn test_single_one() {
        let matrix = vec![vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 1);
    }

    #[test]
    fn test_all_ones() {
        let matrix = vec![vec!['1', '1'], vec!['1', '1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 4);
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![vec!['1', '1', '1', '1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 4);
    }

    #[test]
    fn test_single_column() {
        let matrix = vec![vec!['1'], vec!['1'], vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 3);
    }
}
