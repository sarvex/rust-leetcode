impl Solution {
    /// Dynamic programming game theory: determine if the current player wins.
    ///
    /// # Intuition
    /// A position is a winning position if there exists at least one move to a losing
    /// position. A position is a losing position if all moves lead to winning positions.
    /// Base case: 0 stones means the current player cannot move, so they lose (false).
    ///
    /// # Approach
    /// Build a DP array where `dp[i]` = true if the player whose turn it is with `i`
    /// stones can win. For each `i`, iterate over all perfect squares `k² <= i`. If
    /// `dp[i - k²]` is false (opponent loses from that state), then `dp[i]` = true.
    ///
    /// # Complexity
    /// - Time: O(n * sqrt(n))
    /// - Space: O(n)
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];

        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                if !dp[i - k * k] {
                    dp[i] = true;
                    break;
                }
                k += 1;
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_1_alice_wins() {
        assert!(Solution::winner_square_game(1));
    }

    #[test]
    fn test_n_2_bob_wins() {
        assert!(!Solution::winner_square_game(2));
    }

    #[test]
    fn test_n_4_perfect_square_alice_wins() {
        assert!(Solution::winner_square_game(4));
    }

    #[test]
    fn test_n_7_bob_wins() {
        // 7 -> 6 (remove 1) -> 5 (remove 1) -> 1 (remove 4) -> 0; alice loses
        assert!(!Solution::winner_square_game(7));
    }

    #[test]
    fn test_n_17_alice_wins() {
        assert!(Solution::winner_square_game(17));
    }

    #[test]
    fn test_large_n() {
        // Boundary: n = 100_000
        // Just verify it runs without panic; result is deterministic
        let _ = Solution::winner_square_game(100_000);
    }
}
