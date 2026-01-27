impl Solution {
    /// Row and column sum precomputation for special position detection.
    ///
    /// # Intuition
    /// A position is special if it contains 1 and is the only 1 in its row
    /// and column. Precomputing row and column sums allows O(1) validation.
    ///
    /// # Approach
    /// 1. Compute sum of each row and column
    /// 2. Count cells where `mat[i][j] == 1` and both row/column sums are 1
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m + n) for row and column sums
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let row_sum: Vec<i32> = (0..m).map(|i| mat[i].iter().sum()).collect();
        let col_sum: Vec<i32> = (0..n).map(|j| (0..m).map(|i| mat[i][j]).sum()).collect();

        (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .filter(|&(i, j)| mat[i][j] == 1 && row_sum[i] == 1 && col_sum[j] == 1)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_special() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
    }

    #[test]
    fn multiple_special() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }

    #[test]
    fn no_special() {
        assert_eq!(Solution::num_special(vec![vec![1, 1], vec![1, 1]]), 0);
    }
}
