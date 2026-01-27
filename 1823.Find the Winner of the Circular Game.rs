impl Solution {
    /// Solves the Josephus problem to find the winner of the circular game.
    ///
    /// # Intuition
    /// The Josephus recurrence gives the zero-indexed survivor position.
    /// Building iteratively from n=1 avoids recursion overhead.
    ///
    /// # Approach
    /// 1. Use the iterative Josephus formula: f(i) = (f(i-1) + k) % i.
    /// 2. Convert the zero-indexed result to one-indexed.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        (2..=n).fold(0, |survivor, i| (survivor + k) % i) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }

    #[test]
    fn test_single_person() {
        assert_eq!(Solution::find_the_winner(1, 1), 1);
    }
}
