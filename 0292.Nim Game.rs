impl Solution {
    /// Determines if the first player wins Nim using modular arithmetic.
    ///
    /// # Intuition
    /// If n is a multiple of 4, any move (1-3 stones) leaves the opponent in
    /// a winning position. Otherwise, the first player can always force a win.
    ///
    /// # Approach
    /// Return `n % 4 != 0`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_of_four_loses() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn not_multiple_of_four_wins() {
        assert!(Solution::can_win_nim(1));
        assert!(Solution::can_win_nim(2));
        assert!(Solution::can_win_nim(3));
    }
}
