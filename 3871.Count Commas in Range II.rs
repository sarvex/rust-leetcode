impl Solution {
    /// Counts the total number of commas used when writing 1..=n in standard formatting.
    ///
    /// # Intuition
    /// A comma is inserted after every three digits from the right, so an integer
    /// with `d` digits contains exactly `(d - 1) / 3` commas. Numbers sharing the
    /// same digit length therefore contribute the same number of commas each, so
    /// instead of iterating over every value in `[1, n]` (up to `10^15`), we group
    /// by digit length and count how many integers fall into each group.
    ///
    /// # Approach
    /// For every digit length `d` (from 1 up to 16, since `n <= 10^15`):
    /// - The `d`-digit integers span `[10^(d-1), 10^d - 1]`.
    /// - Intersect that range with `[1, n]` to get the count of qualifying numbers.
    /// - Multiply that count by `(d - 1) / 3`, the commas each such number uses.
    ///
    /// Summing across all digit lengths yields the total. Arithmetic is done in
    /// `i128` to avoid overflow when multiplying large counts by comma counts.
    ///
    /// # Complexity
    /// - Time: O(1) — at most 16 digit-length buckets are inspected.
    /// - Space: O(1)
    pub fn count_commas(n: i64) -> i64 {
        let n = n as i128;
        let mut total: i128 = 0;
        let mut low: i128 = 1; // 10^(d-1), starting at d = 1

        for d in 1..=16u32 {
            let high = low * 10 - 1; // 10^d - 1
            let hi = high.min(n);
            if hi >= low {
                let count = hi - low + 1;
                total += count * (((d - 1) / 3) as i128);
            }
            if high >= n {
                break;
            }
            low *= 10;
        }

        total as i64
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
    fn test_seven_digit_boundary() {
        // 1_000_000 has 7 digits -> (7-1)/3 = 2 commas.
        // Numbers 1000..=999999 each have 1 comma: 999000 numbers -> 999000 commas.
        // Plus 1_000_000 itself contributes 2.
        assert_eq!(Solution::count_commas(1_000_000), 999_000 + 2);
    }

    #[test]
    fn test_brute_force_agreement() {
        fn commas_in(x: i64) -> i64 {
            let digits = x.to_string().len() as i64;
            (digits - 1) / 3
        }
        for n in 1..=5000i64 {
            let expected: i64 = (1..=n).map(commas_in).sum();
            assert_eq!(Solution::count_commas(n), expected, "mismatch at n = {n}");
        }
    }

    #[test]
    fn test_max_constraint() {
        // n = 10^15 has 16 digits. Verify it runs and returns a sane large value.
        let result = Solution::count_commas(1_000_000_000_000_000);
        // 10^15 itself: (16-1)/3 = 5 commas. Sanity: result must be positive.
        assert!(result > 0);
    }
}
