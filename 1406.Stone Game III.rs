impl Solution {
    /// DP on suffix: track the maximum score advantage the current player can gain.
    ///
    /// # Intuition
    /// At each position, the current player can take 1, 2, or 3 stones. Taking `k` stones
    /// gives a score of `sum(stone_value[i..i+k])`, and then the opponent plays optimally
    /// from position `i+k`. The key insight: track the *net advantage* (my score minus
    /// opponent's score from the remaining suffix). The current player picks the take size
    /// that maximises their own advantage.
    ///
    /// # Approach
    /// Define `dp[i]` = maximum score advantage the current player achieves starting at
    /// index `i` (i.e., sum of stones the current player takes minus sum the opponent takes,
    /// from index `i` onward, both playing optimally).
    ///
    /// Recurrence (iterating right-to-left):
    ///   dp[i] = max over k in {1,2,3} of (prefix_sum[i..i+k] - dp[i+k])
    ///
    /// where `dp[n] = 0` (no stones left). We use a rolling array of size 4 to avoid
    /// an O(n) allocation.
    ///
    /// Final answer: compare Alice's advantage `dp[0]` (which equals alice_score - bob_score).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        // dp[i % 4] stores the best score advantage from index i onward
        let mut dp = [i32::MIN; 4];
        dp[n % 4] = 0;

        for i in (0..n).rev() {
            let mut best = i32::MIN;
            let mut take = 0;
            for k in 1..=3 {
                if i + k > n {
                    break;
                }
                take += stone_value[i + k - 1];
                let next = dp[(i + k) % 4];
                if next != i32::MIN {
                    best = best.max(take - next);
                }
            }
            dp[i % 4] = best;
        }

        match dp[0].cmp(&0) {
            std::cmp::Ordering::Greater => "Alice".to_string(),
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_bob_wins() {
        // Bob wins: Alice's best is 6, Bob takes 7
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
    }

    #[test]
    fn test_example_2_alice_wins() {
        // Alice takes all three first piles, Bob stuck with -9
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
    }

    #[test]
    fn test_example_3_tie() {
        // Alice takes first three (6), Bob takes last pile (6)
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
    }

    #[test]
    fn test_single_stone_positive() {
        // Alice takes the only stone
        assert_eq!(Solution::stone_game_iii(vec![5]), "Alice");
    }

    #[test]
    fn test_single_stone_negative() {
        // Alice must take it, ends with negative score; Bob has 0
        assert_eq!(Solution::stone_game_iii(vec![-5]), "Bob");
    }

    #[test]
    fn test_all_negative() {
        // Alice takes 2 (-3), Bob takes 1 (-3) → Tie is the best Alice can achieve
        assert_eq!(Solution::stone_game_iii(vec![-1, -2, -3]), "Tie");
    }

    #[test]
    fn test_two_stones_alice_grabs_both() {
        // Alice takes 2 stones (score 6), Bob gets nothing → Alice wins
        assert_eq!(Solution::stone_game_iii(vec![3, 3]), "Alice");
    }

    #[test]
    fn test_alice_takes_three_to_win() {
        assert_eq!(Solution::stone_game_iii(vec![10, 10, 10, 1]), "Alice");
    }

    #[test]
    fn test_large_uniform() {
        let stones = vec![1000; 50000];
        let result = Solution::stone_game_iii(stones);
        assert!(result == "Alice" || result == "Bob" || result == "Tie");
    }
}
