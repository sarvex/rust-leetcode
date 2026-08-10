impl Solution {
    /// Suffix-sum DP: maximize Alice's stones in optimal two-player Stone Game II.
    ///
    /// # Intuition
    /// Each player wants to maximize their own total. Because both play optimally, what Alice
    /// gains is the total of all stones minus what Bob gains — and Bob plays with the same
    /// optimizing logic on his turns. A memoized DP over (index, M) gives the maximum stones
    /// the current player can collect from `piles[index..]` when the current `M` value is `m`.
    ///
    /// # Approach
    /// 1. Build a suffix-sum array so the total stones from any index onward is O(1).
    /// 2. Define `dp[i][m]` = max stones the current player can take from `piles[i..]` with
    ///    current multiplier `m`.
    /// 3. Base case: if `i + 2*m >= n`, the current player takes everything → `suffix[i]`.
    /// 4. Transition: try every `x` in `1..=2*m`. The current player takes `suffix[i] -
    ///    suffix[i+x]` implicitly; the opponent then plays `dp[i+x][max(m,x)]`, so we want
    ///    the `x` that maximises `suffix[i] - dp[i+x][max(m,x)]`.
    /// 5. Return `dp[0][1]`.
    ///
    /// # Complexity
    /// - Time: O(n³) — O(n²) states × O(n) transitions
    /// - Space: O(n²) for the DP table
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        // suffix[i] = sum of piles[i..]
        let mut suffix = vec![0i32; n + 1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] + piles[i];
        }

        // dp[i][m] = max stones current player collects from index i with multiplier m
        // m ranges from 1 to n; cap at n to avoid oversized table
        let mut dp = vec![vec![0i32; n + 1]; n + 1];

        for i in (0..n).rev() {
            for m in 1..=n {
                if i + 2 * m >= n {
                    // Can take all remaining piles
                    dp[i][m] = suffix[i];
                } else {
                    // Try each valid x; maximise current player's haul
                    dp[i][m] = (1..=2 * m)
                        .map(|x| suffix[i] - dp[i + x][m.max(x)])
                        .max()
                        .unwrap_or(0);
                }
            }
        }

        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Alice takes 1, Bob takes 2, Alice takes 2 → 2+4+4 = 10
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

    #[test]
    fn test_example_2() {
        // Alice takes 5 piles (1+2+3+4+5=15), Bob must leave 100 behind and
        // Alice takes it; optimal: Alice gets 1+3+100=104
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }

    #[test]
    fn test_single_pile() {
        // Alice must take the only pile
        assert_eq!(Solution::stone_game_ii(vec![5]), 5);
    }

    #[test]
    fn test_two_piles() {
        // With M=1 Alice can take 1 or 2 piles (2*M=2 >= n=2), so she grabs both → 13
        assert_eq!(Solution::stone_game_ii(vec![10, 3]), 13);
    }

    #[test]
    fn test_equal_piles() {
        // Symmetric game: each player gets half → 6
        assert_eq!(Solution::stone_game_ii(vec![3, 3, 3, 3]), 6);
    }

    #[test]
    fn test_large_last_pile() {
        // Bob can always position to take the 100-pile; Alice nets only 3
        assert_eq!(Solution::stone_game_ii(vec![1, 1, 1, 1, 100]), 3);
    }

    #[test]
    fn test_large_first_pile() {
        // Alice takes pile[0]=100 immediately (M=1, takes 1 pile) → 100+1+1=102
        assert_eq!(Solution::stone_game_ii(vec![100, 1, 1, 1, 1]), 102);
    }

    #[test]
    fn test_increasing_piles() {
        assert_eq!(
            Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            26
        );
    }
}
