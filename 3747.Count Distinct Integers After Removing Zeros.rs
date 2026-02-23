impl Solution {
    /// Counts distinct integers obtainable by deleting all zero digits.
    ///
    /// # Intuition
    /// Let `f(x)` be `x` with all `'0'` digits removed. Every produced value has no
    /// zero digits. Also, if `f(x) = y`, then `y <= x` (inserting zeros never makes
    /// the value smaller), so `y <= n`.
    ///
    /// Conversely, any zero-free integer `y <= n` is produced by taking `x = y`.
    /// Therefore, the problem is exactly counting integers in `[1, n]` whose decimal
    /// representation contains no zero.
    ///
    /// # Approach
    /// - Count all valid numbers with fewer digits than `n`: for length `len`, there
    ///   are `9^len` choices (each digit in `[1..=9]`).
    /// - Count valid numbers with the same length as `n` using a digit DP scan:
    ///   - At each position, add choices smaller than current digit but non-zero.
    ///   - If current digit is `0`, we cannot continue matching the prefix.
    /// - If `n` itself has no zero digits, include it.
    ///
    /// # Complexity
    /// - Time: O(log10(n))
    /// - Space: O(log10(n))
    pub fn count_distinct(n: i64) -> i64 {
        let digits = n.to_string().into_bytes();
        let digit_count = digits.len();

        let mut powers_of_nine = vec![1_i64; digit_count + 1];
        (1..=digit_count).for_each(|idx| {
            powers_of_nine[idx] = powers_of_nine[idx - 1] * 9;
        });

        let mut distinct_count = (1..digit_count)
            .map(|len| powers_of_nine[len])
            .sum::<i64>();

        for (idx, &byte) in digits.iter().enumerate() {
            let digit = i64::from(byte - b'0');
            let remaining = digit_count - idx - 1;
            let smaller_non_zero_choices = if digit == 0 { 0 } else { digit - 1 };

            distinct_count += smaller_non_zero_choices * powers_of_nine[remaining];

            if digit == 0 {
                return distinct_count;
            }
        }

        distinct_count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_distinct(10), 9);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_distinct(3), 3);
    }

    #[test]
    fn test_power_of_ten_boundary() {
        assert_eq!(Solution::count_distinct(1_000), 819);
    }

    #[test]
    fn test_with_internal_zeros() {
        assert_eq!(Solution::count_distinct(1_010), 819);
    }

    #[test]
    fn test_upper_constraint() {
        let expected = (1..=15_u32).map(|exp| 9_i64.pow(exp)).sum::<i64>();
        assert_eq!(Solution::count_distinct(1_000_000_000_000_000), expected);
    }
}
