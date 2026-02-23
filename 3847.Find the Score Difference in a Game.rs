impl Solution {
    /// Simulates the game and returns first player's score minus second player's score.
    ///
    /// # Intuition
    /// Two swap rules determine who is active before each game: an odd-value swap and a
    /// periodic 6th-game swap. Since we only need the difference, track a single signed
    /// accumulator — add when the first player is active, subtract otherwise.
    ///
    /// # Approach
    /// Fold over enumerated values carrying `(difference, first_active)`. For each game,
    /// apply the odd swap, then the 6th-game swap, then accumulate `+pts` or `-pts`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold((0, true), |(diff, mut first_active), (i, pts)| {
                if *pts & 1 == 1 {
                    first_active = !first_active;
                }
                if i % 6 == 5 {
                    first_active = !first_active;
                }
                (
                    if first_active {
                        diff + *pts
                    } else {
                        diff - *pts
                    },
                    first_active,
                )
            })
            .0
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
