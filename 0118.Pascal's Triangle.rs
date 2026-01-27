impl Solution {
    /// Row-by-row construction of Pascal's triangle.
    ///
    /// # Intuition
    /// Each row starts and ends with 1. Interior elements are the sum
    /// of the two elements directly above from the previous row.
    ///
    /// # Approach
    /// For each row index `i`, create a vector of ones with length `i + 1`.
    /// Fill interior positions from the previous row's adjacent pair sums.
    /// Collect all rows into the result.
    ///
    /// # Complexity
    /// - Time: O(n^2) — filling all elements of the triangle
    /// - Space: O(n^2) — storing the triangle
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        (0..num_rows as usize).fold(Vec::new(), |mut triangle, i| {
            let mut row = vec![1; i + 1];
            for j in 1..i {
                row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
            triangle.push(row);
            triangle
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_rows() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        );
    }

    #[test]
    fn single_row() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn two_rows() {
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    }
}
