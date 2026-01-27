use std::cmp::Ordering;

impl Solution {
    /// Staircase search from top-right corner of a sorted 2D matrix.
    ///
    /// # Intuition
    /// Starting from the top-right corner, each comparison eliminates an
    /// entire row or column: move down if the target is larger, left if
    /// smaller.
    ///
    /// # Approach
    /// Initialize at row 0, column n-1. Compare the current element with
    /// the target. On equality, return true. If the element is less than
    /// the target, move down. If greater, move left. Return false when
    /// out of bounds.
    ///
    /// # Complexity
    /// - Time: O(m + n) — at most m + n steps
    /// - Space: O(1) — scalar pointers only
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut row, mut col) = (0, n);

        while row < m && col > 0 {
            match matrix[row][col - 1].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => row += 1,
                Ordering::Greater => col -= 1,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn target_not_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn single_element_found() {
        assert!(Solution::search_matrix(vec![vec![1]], 1));
    }

    #[test]
    fn single_element_not_found() {
        assert!(!Solution::search_matrix(vec![vec![1]], 2));
    }
}
