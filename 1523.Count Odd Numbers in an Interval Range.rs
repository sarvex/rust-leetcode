impl Solution {
    /// Bit-shift formula for counting odd numbers in a range.
    ///
    /// # Intuition
    /// The count of odd numbers in `[0, n]` is `(n + 1) / 2`. For `[low, high]`,
    /// subtract the count below `low`: `(high + 1) / 2 - low / 2`.
    ///
    /// # Approach
    /// 1. Use integer division: `((high + 1) >> 1) - (low >> 1)`
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn count_odds(low: i32, high: i32) -> i32 {
        ((high + 1) >> 1) - (low >> 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_3_to_7() {
        assert_eq!(Solution::count_odds(3, 7), 3);
    }

    #[test]
    fn even_bounds() {
        assert_eq!(Solution::count_odds(8, 10), 1);
    }

    #[test]
    fn single_odd() {
        assert_eq!(Solution::count_odds(3, 3), 1);
    }

    #[test]
    fn single_even() {
        assert_eq!(Solution::count_odds(4, 4), 0);
    }
}
