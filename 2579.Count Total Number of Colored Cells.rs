impl Solution {
    /// Count colored cells after n minutes using the diamond formula.
    ///
    /// # Intuition
    /// The colored region forms a diamond shape. After n steps, the total
    /// cells equal 2n(n-1) + 1.
    ///
    /// # Approach
    /// Direct formula: 2 * n * (n - 1) + 1.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * (n - 1) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_minute() {
        assert_eq!(Solution::colored_cells(1), 1);
    }

    #[test]
    fn test_two_minutes() {
        assert_eq!(Solution::colored_cells(2), 5);
    }

    #[test]
    fn test_three_minutes() {
        assert_eq!(Solution::colored_cells(3), 13);
    }
}
