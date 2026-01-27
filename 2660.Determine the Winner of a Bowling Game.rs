impl Solution {
    /// Determine the bowling game winner by comparing weighted scores.
    ///
    /// # Intuition
    /// A pin score is doubled if a strike (10) occurred in the previous two turns.
    /// Compute weighted scores for both players and compare.
    ///
    /// # Approach
    /// 1. Define a scoring closure that checks the previous two rounds for a strike
    /// 2. Apply to both players and compare totals
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let score = |arr: &[i32]| -> i32 {
            arr.iter()
                .enumerate()
                .map(|(i, &val)| {
                    let has_strike = (i >= 1 && arr[i - 1] == 10) || (i >= 2 && arr[i - 2] == 10);
                    if has_strike {
                        2 * val
                    } else {
                        val
                    }
                })
                .sum()
        };

        let a = score(&player1);
        let b = score(&player2);

        match a.cmp(&b) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player1_wins() {
        assert_eq!(Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]), 1);
    }

    #[test]
    fn test_tie() {
        assert_eq!(Solution::is_winner(vec![1, 2], vec![2, 1]), 0);
    }

    #[test]
    fn test_player2_wins() {
        assert_eq!(Solution::is_winner(vec![1, 1], vec![10, 5]), 2);
    }
}
