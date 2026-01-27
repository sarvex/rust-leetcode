impl Solution {
    /// Determines if a number is strictly palindromic (always false).
    ///
    /// # Intuition
    /// No integer n >= 4 is palindromic in base (n - 2), because n in base (n - 2)
    /// is always "12", which is never a palindrome. The answer is always false.
    ///
    /// # Approach
    /// Return false directly — mathematical proof eliminates all candidates.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_strictly_palindromic(_n: i32) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(!Solution::is_strictly_palindromic(9));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::is_strictly_palindromic(4));
    }

    #[test]
    fn test_large() {
        assert!(!Solution::is_strictly_palindromic(100_000));
    }
}
