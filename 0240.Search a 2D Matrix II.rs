use std::cmp::Ordering;

impl Solution {
    /// Searches a row-sorted and column-sorted matrix from the top-right corner.
    ///
    /// # Intuition
    /// Starting from the top-right, each comparison eliminates an entire row
    /// or column, achieving O(m + n) time.
    ///
    /// # Approach
    /// 1. Start at top-right corner (row 0, last column).
    /// 2. If target < current, move left (eliminate column).
    /// 3. If target > current, move down (eliminate row).
    /// 4. If equal, return true.
    ///
    /// # Complexity
    /// - Time: O(m + n)
    /// - Space: O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut row, mut col) = (0, n);
        while row < m && col > 0 {
            match target.cmp(&matrix[row][col - 1]) {
                Ordering::Less => col -= 1,
                Ordering::Greater => row += 1,
                Ordering::Equal => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_in_matrix() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix(matrix, 5));
    }

    #[test]
    fn not_found() {
        let matrix = vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19]];
        assert!(!Solution::search_matrix(matrix, 20));
    }
}
