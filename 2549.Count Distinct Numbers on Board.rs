impl Solution {
    /// Counts distinct numbers remaining on the board after infinite operations.
    ///
    /// # Intuition
    /// After placing n on the board, any number x where n % x == 1 gets added.
    /// This cascades: n-1 is always added (since n % (n-1) == 1 for n > 2),
    /// then n-2, etc. Eventually all numbers from 2 to n appear.
    ///
    /// # Approach
    /// For n == 1, only 1 remains. For n >= 2, all integers 2..=n appear,
    /// giving n-1 distinct values.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn distinct_integers(n: i32) -> i32 {
        1.max(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_equals_one() {
        assert_eq!(Solution::distinct_integers(1), 1);
    }

    #[test]
    fn test_n_equals_two() {
        assert_eq!(Solution::distinct_integers(2), 1);
    }

    #[test]
    fn test_n_equals_five() {
        assert_eq!(Solution::distinct_integers(5), 4);
    }

    #[test]
    fn test_large_n() {
        assert_eq!(Solution::distinct_integers(100), 99);
    }
}
