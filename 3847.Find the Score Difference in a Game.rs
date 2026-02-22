impl Solution {
    /// Simulates the game: apply odd and 6th-game swaps, then active player gains points.
    ///
    /// # Intuition
    /// The active player is determined before each game by two rules applied in order: swap if
    /// points are odd, then swap on every 6th game (indices 5, 11, 17, ...). The active player
    /// then gains that game's points. Simulate once and return first player's score minus second's.
    ///
    /// # Approach
    /// 1. First player starts active (`first_active = true`), both scores start at 0.
    /// 2. For each game index `i`: if `nums[i]` is odd, flip active; if `i % 6 == 5`, flip again.
    /// 3. Add `nums[i]` to the active player's score.
    /// 4. Return `score_first - score_second`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    ///
    /// # Panics
    /// Never panics; indices and values are within constraints.
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        let mut first_active = true;
        let (mut score_first, mut score_second) = (0i32, 0i32);

        for (i, &pts) in nums.iter().enumerate() {
            if pts & 1 == 1 {
                first_active = !first_active;
            }
            if i % 6 == 5 {
                first_active = !first_active;
            }
            if first_active {
                score_first += pts;
            } else {
                score_second += pts;
            }
        }

        score_first - score_second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::score_difference(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::score_difference(vec![2, 4, 2, 1, 2, 1]), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::score_difference(vec![1]), -1);
    }

    #[test]
    fn test_even_only_first_stays_active() {
        assert_eq!(Solution::score_difference(vec![2, 4, 6]), 12);
    }

    #[test]
    fn test_sixth_game_double_swap() {
        // Games 0..=4: first gets all. Game 5: odd swap then 6th-game swap -> second gets it.
        let nums: Vec<i32> = (0..6).map(|_| 2).collect();
        assert_eq!(Solution::score_difference(nums), 10 - 2);
    }
}
