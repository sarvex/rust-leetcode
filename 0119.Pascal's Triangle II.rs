impl Solution {
    /// Returns the k-th row of Pascal's triangle using in-place updates.
    ///
    /// # Intuition
    /// Each row can be computed from the previous by updating values right-to-left,
    /// avoiding the need to store the entire triangle.
    ///
    /// # Approach
    /// 1. Initialize a vector of ones with length `row_index + 1`.
    /// 2. For each row from 2 onward, update elements from right to left
    ///    by adding the element to its left neighbor.
    ///
    /// # Complexity
    /// - Time: O(k^2) where k is row_index
    /// - Space: O(k) for the single row
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = (row_index + 1) as usize;
        let mut row = vec![1; n];
        for i in 2..n {
            for j in (1..i).rev() {
                row[j] += row[j - 1];
            }
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_zero_returns_single_one() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn row_one_returns_pair() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }

    #[test]
    fn row_three_returns_correct_values() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn row_four_returns_correct_values() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }
}
