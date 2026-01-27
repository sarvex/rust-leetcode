impl Solution {
    /// Computes the probability of a knight remaining on the board after k moves.
    ///
    /// # Intuition
    /// DP where f[step][i][j] is the probability of being at (i,j) after
    /// `step` moves while staying on the board.
    ///
    /// # Approach
    /// 1. Initialize f[0][i][j] = 1.0 for all cells.
    /// 2. For each step, sum contributions from all 8 knight-move predecessors.
    /// 3. Each predecessor contributes 1/8 of its probability.
    ///
    /// # Complexity
    /// - Time: O(k × n²)
    /// - Space: O(k × n²)
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![vec![0.0f64; n]; n]; k + 1];
        for i in 0..n {
            for j in 0..n {
                dp[0][i][j] = 1.0;
            }
        }
        let moves: [(isize, isize); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];
        for step in 1..=k {
            for i in 0..n {
                for j in 0..n {
                    for &(dx, dy) in &moves {
                        let x = i as isize + dx;
                        let y = j as isize + dy;
                        if x >= 0 && x < n as isize && y >= 0 && y < n as isize {
                            dp[step][i][j] += dp[step - 1][x as usize][y as usize] / 8.0;
                        }
                    }
                }
            }
        }
        dp[k][row as usize][column as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::knight_probability(3, 2, 0, 0);
        assert!((result - 0.0625).abs() < 1e-5);
    }

    #[test]
    fn test_zero_moves() {
        assert!((Solution::knight_probability(1, 0, 0, 0) - 1.0).abs() < 1e-5);
    }
}
