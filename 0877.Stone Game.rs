impl Solution {
    /// Alice always wins via the mathematical guarantee for even-length piles with odd sum.
    ///
    /// # Intuition
    /// With an even number of piles and an odd total sum, Alice can always win by a
    /// parity argument: she can pre-commit to taking all even-indexed or all odd-indexed
    /// piles (whichever sums more) and enforce that strategy from the first move. Since
    /// total is odd one parity sum strictly exceeds the other, so Alice always wins.
    ///
    /// Equivalently, applying interval DP — dp[i][j] = max score advantage the current
    /// player achieves over piles[i..=j] — yields dp[0][n-1] > 0 for all valid inputs.
    ///
    /// # Approach
    /// Return `true` unconditionally; the constraints guarantee Alice always wins.
    /// An interval-DP proof is included via the equivalent `predict_the_winner` logic:
    ///   dp[i][i] = piles[i]
    ///   dp[i][j] = max(piles[i] - dp[i+1][j], piles[j] - dp[i][j-1])
    ///   Alice wins iff dp[0][n-1] > 0  (always true under these constraints)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn stone_game(_piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Alice takes first 5: [3,4,5] -> Bob takes 3 or 5, Alice wins either way
        assert!(Solution::stone_game(vec![5, 3, 4, 5]));
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::stone_game(vec![3, 7, 2, 3]));
    }

    #[test]
    fn test_two_piles() {
        // Alice takes the larger pile; always wins with two piles (sum is odd)
        assert!(Solution::stone_game(vec![1, 2]));
    }

    #[test]
    fn test_large_disparity() {
        assert!(Solution::stone_game(vec![1, 100, 1, 100]));
    }

    #[test]
    fn test_minimum_constraints() {
        assert!(Solution::stone_game(vec![1, 2]));
    }
}
