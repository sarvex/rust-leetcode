impl Solution {
    /// Finds the minimum path sum from top to bottom of a triangle using bottom-up DP.
    ///
    /// # Intuition
    /// Working from the bottom row upward, each element only needs to consider
    /// the minimum of its two adjacent elements in the row below.
    ///
    /// # Approach
    /// 1. Create a DP array initialized to zeros with length `n + 1`.
    /// 2. Iterate from the bottom row to the top.
    /// 3. For each position, store the minimum of the two children plus the current value.
    /// 4. The answer is `f[0]` after processing all rows.
    ///
    /// # Complexity
    /// - Time: O(n^2) where n is the number of rows
    /// - Space: O(n) for the single DP array
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            for j in 0..=i {
                dp[j] = dp[j].min(dp[j + 1]) + triangle[i][j];
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_triangle() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }

    #[test]
    fn two_rows() {
        let triangle = vec![vec![-1], vec![2, 3]];
        assert_eq!(Solution::minimum_total(triangle), 1);
    }
}
