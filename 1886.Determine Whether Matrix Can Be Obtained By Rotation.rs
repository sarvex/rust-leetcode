impl Solution {
    /// Checks if the matrix equals the target after 0, 90, 180, or 270 degree rotation.
    ///
    /// # Intuition
    /// Rather than physically rotating, check all four rotation mappings
    /// simultaneously in a single pass over the matrix.
    ///
    /// # Approach
    /// 1. For each cell (i, j), verify against all four rotation positions.
    /// 2. Track which rotations remain valid using boolean flags.
    /// 3. Return true if any rotation matches completely.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(1)
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        let mut valid = [true; 4];

        for i in 0..n {
            for j in 0..n {
                if valid[0] && mat[i][j] != target[i][j] {
                    valid[0] = false;
                }
                if valid[1] && mat[i][j] != target[j][n - 1 - i] {
                    valid[1] = false;
                }
                if valid[2] && mat[i][j] != target[n - 1 - i][n - 1 - j] {
                    valid[2] = false;
                }
                if valid[3] && mat[i][j] != target[n - 1 - j][i] {
                    valid[3] = false;
                }
            }
        }

        valid.iter().any(|&v| v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_match() {
        assert!(Solution::find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn test_no_rotation_match() {
        assert!(!Solution::find_rotation(
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn test_identity_rotation() {
        assert!(Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        ));
    }
}
