impl Solution {
    /// Prefix Sum with Incremental Expansion
    ///
    /// # Intuition
    /// For each cell, try expanding the square size starting from the current
    /// best + 1. Since square sums increase monotonically with size, break
    /// immediately when threshold is exceeded.
    ///
    /// # Approach
    /// 1. Build a 1-indexed prefix sum matrix for O(1) rectangular sum queries
    /// 2. For each cell, expand square size from (result + 1) until invalid
    /// 3. Early break when sum exceeds threshold exploits monotonicity
    ///
    /// # Complexity
    /// - Time: O(m * n * min(m, n)) worst case, but fast in practice
    /// - Space: O(m * n)
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut prefix = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1]
                    - prefix[i - 1][j - 1] + mat[i - 1][j - 1];
            }
        }

        let max_side = m.min(n);
        let mut result = 0;

        for i in 1..=m {
            for j in 1..=n {
                for k in (result + 1)..=max_side {
                    let (x2, y2) = (i + k - 1, j + k - 1);
                    if x2 > m || y2 > n {
                        break;
                    }
                    let sum = prefix[x2][y2] - prefix[i - 1][y2]
                        - prefix[x2][j - 1] + prefix[i - 1][j - 1];
                    if sum <= threshold {
                        result = k;
                    } else {
                        break;
                    }
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mat = vec![
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
            vec![1, 1, 3, 2, 4, 3, 2],
        ];
        assert_eq!(Solution::max_side_length(mat, 4), 2);
    }

    #[test]
    fn test_example_2() {
        let mat = vec![
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2],
        ];
        assert_eq!(Solution::max_side_length(mat, 1), 0);
    }

    #[test]
    fn test_single_element_valid() {
        let mat = vec![vec![5]];
        assert_eq!(Solution::max_side_length(mat, 5), 1);
    }

    #[test]
    fn test_single_element_invalid() {
        let mat = vec![vec![5]];
        assert_eq!(Solution::max_side_length(mat, 4), 0);
    }

    #[test]
    fn test_entire_matrix() {
        let mat = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(Solution::max_side_length(mat, 9), 3);
    }

    #[test]
    fn test_large_threshold() {
        let mat = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        assert_eq!(Solution::max_side_length(mat, 100000), 2);
    }
}
