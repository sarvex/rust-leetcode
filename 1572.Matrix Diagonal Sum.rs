impl Solution {
    /// Sum both diagonals avoiding double-counting the center.
    ///
    /// # Intuition
    /// The primary diagonal has indices `(i, i)` and the secondary `(i, n-1-i)`.
    /// When n is odd, the center element lies on both diagonals and must be
    /// counted only once.
    ///
    /// # Approach
    /// 1. Iterate rows, summing `mat[i][i]` and `mat[i][n-1-i]`
    /// 2. Skip the secondary when `i == n-1-i` (center of odd-size matrix)
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        (0..n)
            .map(|i| {
                let j = n - 1 - i;
                mat[i][i] + if j != i { mat[i][j] } else { 0 }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_by_three() {
        assert_eq!(
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }

    #[test]
    fn even_size() {
        assert_eq!(
            Solution::diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
            ]),
            8
        );
    }
}
