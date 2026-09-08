impl Solution {
    /// Counts the total number of commas used when writing 1..=n in standard formatting.
    ///
    /// # Intuition
    /// A comma is inserted after every three digits from the right, so a number
    /// with `d` digits contains `(d - 1) / 3` commas. Since `n <= 10^5`, every
    /// integer in `[1, n]` has at most 6 digits, meaning any value with 4, 5, or
    /// 6 digits (i.e. `>= 1000`) contains exactly one comma, and values below
    /// 1000 contain none.
    ///
    /// # Approach
    /// Only integers from 1000 to n contribute a comma, each contributing exactly
    /// one. The count is therefore `max(0, n - 1000 + 1)` when `n >= 1000`.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn count_commas(n: i32) -> i32 {
        (n - 999).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_commas(1002), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_commas(998), 0);
    }

    #[test]
    fn test_exactly_1000() {
        assert_eq!(Solution::count_commas(1000), 1);
    }

    #[test]
    fn test_just_below_threshold() {
        assert_eq!(Solution::count_commas(999), 0);
    }

    #[test]
    fn test_min_constraint() {
        assert_eq!(Solution::count_commas(1), 0);
    }

    #[test]
    fn test_max_constraint() {
        // n = 10^5 = 100000; commas from 1000..=100000 = 99001
        assert_eq!(Solution::count_commas(100_000), 99_001);
    }
}
