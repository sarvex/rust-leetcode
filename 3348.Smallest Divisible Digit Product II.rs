pub struct Solution;

/// Tracks required counts of prime factors 2, 3, 5, 7 for divisibility.
#[derive(Clone, Copy, Debug, Default)]
struct PrimeFactorCount {
    twos: u8,
    threes: u8,
    fives: u8,
    sevens: u8,
}

/// Precomputed factor contributions for digits 0-9: (twos, threes, fives, sevens).
const DIGIT_PRIME_FACTORS: [(u8, u8, u8, u8); 10] = [
    (0, 0, 0, 0), // 0
    (0, 0, 0, 0), // 1
    (1, 0, 0, 0), // 2 = 2^1
    (0, 1, 0, 0), // 3 = 3^1
    (2, 0, 0, 0), // 4 = 2^2
    (0, 0, 1, 0), // 5 = 5^1
    (1, 1, 0, 0), // 6 = 2^1 * 3^1
    (0, 0, 0, 1), // 7 = 7^1
    (3, 0, 0, 0), // 8 = 2^3
    (0, 2, 0, 0), // 9 = 3^2
];

impl Solution {
    /// Finds the smallest zero-free number >= num with digit product divisible by t.
    ///
    /// # Intuition
    /// The digit product can only contain prime factors 2, 3, 5, 7 (from digits 2-9).
    /// If t has any prime factor other than these, no solution exists. We track remaining
    /// factor requirements as we process each digit prefix.
    ///
    /// # Approach
    /// 1. Validate t contains only factors 2, 3, 5, 7. If not, return "-1".
    /// 2. Calculate the minimum number of digits needed to satisfy t.
    /// 3. Build prefix factor-requirement snapshots digit by digit (stopping at any zero).
    /// 4. If the original number already satisfies and contains no zeros, return it.
    /// 5. Try incrementing each position from right to left, filling the suffix optimally.
    /// 6. Fall back to a longer number (leading '1' + optimal suffix) if no same-length
    ///    solution exists.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of digits in num
    /// - Space: O(n) for storing prefix requirements and result
    pub fn smallest_number(num: String, t: i64) -> String {
        let required_factors = Self::extract_prime_factors(t);
        if required_factors.twos == u8::MAX {
            return "-1".to_string();
        }

        let input_bytes = num.as_bytes();
        let input_length = input_bytes.len();
        let min_digits_for_target = Self::compute_minimum_digits_needed(required_factors);

        // Find first zero — zeros make digit product 0 which can't be non-zero divisible.
        let first_zero_pos = input_bytes
            .iter()
            .position(|&b| b == b'0')
            .unwrap_or(input_length);

        // Build prefix factor-requirement snapshots: prefix_factors[i] = factors still
        // needed after consuming input_bytes[0..i].
        let mut prefix_factors = Vec::with_capacity(first_zero_pos + 1);
        prefix_factors.push(required_factors);

        let remaining_factors =
            input_bytes[..first_zero_pos]
                .iter()
                .fold(required_factors, |factors, &digit_byte| {
                    let digit = digit_byte - b'0';
                    let new_factors = Self::subtract_digit_contribution(factors, digit);
                    prefix_factors.push(new_factors);
                    new_factors
                });

        // If no zeros and original already satisfies, return it as-is.
        if first_zero_pos == input_length
            && Self::compute_minimum_digits_needed(remaining_factors) == 0
        {
            return num;
        }

        // Try incrementing from rightmost possible position leftward.
        let max_changeable_pos = first_zero_pos.min(input_length - 1);

        for change_pos in (0..=max_changeable_pos).rev() {
            let factors_before = prefix_factors[change_pos];
            let suffix_len = input_length - 1 - change_pos;
            let current_digit = input_bytes[change_pos] - b'0';

            for new_digit in (current_digit + 1)..=9 {
                let factors_after = Self::subtract_digit_contribution(factors_before, new_digit);
                let min_suffix_digits = Self::compute_minimum_digits_needed(factors_after);

                if min_suffix_digits <= suffix_len {
                    return Self::build_result(
                        &input_bytes[..change_pos],
                        new_digit,
                        factors_after,
                        suffix_len,
                    );
                }
            }
        }

        // No same-length solution. Find the shortest possible longer number,
        // then return the lexicographically smallest one of that length.
        //
        // For each possible leading digit d (1..9), the total length needed is
        //   1 + compute_minimum_digits_needed(required_factors after subtracting d).
        // We pick the smallest such total length L >= input_length + 1, then among
        // all leading digits that achieve length L, pick the smallest d and build
        // the optimal suffix.
        let min_total_len = (1u8..=9)
            .map(|d| {
                let after = Self::subtract_digit_contribution(required_factors, d);
                1 + Self::compute_minimum_digits_needed(after)
            })
            .min()
            .unwrap_or(1 + min_digits_for_target)
            .max(input_length + 1);

        for leading_digit in 1u8..=9 {
            let factors_after_lead =
                Self::subtract_digit_contribution(required_factors, leading_digit);
            let needed = Self::compute_minimum_digits_needed(factors_after_lead);
            let total_len = (1 + needed).max(input_length + 1);
            if total_len == min_total_len {
                let suffix_len = min_total_len - 1;
                return Self::build_result(&[], leading_digit, factors_after_lead, suffix_len);
            }
        }
        // Unreachable for valid inputs.
        "-1".to_string()
    }

    /// Extracts prime factors 2, 3, 5, 7 from `value`.
    /// Sets `twos = u8::MAX` as a sentinel if `value` has any other prime factor.
    #[inline(always)]
    fn extract_prime_factors(mut value: i64) -> PrimeFactorCount {
        let mut factors = PrimeFactorCount::default();

        while value % 2 == 0 {
            factors.twos += 1;
            value /= 2;
        }
        while value % 3 == 0 {
            factors.threes += 1;
            value /= 3;
        }
        while value % 5 == 0 {
            factors.fives += 1;
            value /= 5;
        }
        while value % 7 == 0 {
            factors.sevens += 1;
            value /= 7;
        }

        if value != 1 {
            factors.twos = u8::MAX; // sentinel: no solution exists
        }
        factors
    }

    /// Subtracts a digit's prime-factor contribution from the remaining requirements.
    #[inline(always)]
    fn subtract_digit_contribution(factors: PrimeFactorCount, digit: u8) -> PrimeFactorCount {
        let (twos, threes, fives, sevens) = DIGIT_PRIME_FACTORS[digit as usize];
        PrimeFactorCount {
            twos: factors.twos.saturating_sub(twos),
            threes: factors.threes.saturating_sub(threes),
            fives: factors.fives.saturating_sub(fives),
            sevens: factors.sevens.saturating_sub(sevens),
        }
    }

    /// Returns the minimum number of digits needed to cover all remaining factor requirements.
    #[inline(always)]
    fn compute_minimum_digits_needed(factors: PrimeFactorCount) -> usize {
        // Each required 7 → digit '7'; each required 5 → digit '5'.
        let mut count = factors.sevens as usize + factors.fives as usize;

        // Pack 3s into 9s (covers 2 threes), pack 2s into 8s (covers 3 twos).
        count += (factors.threes / 2) as usize + (factors.twos / 3) as usize;

        let rem_threes = factors.threes % 2;
        let rem_twos = factors.twos % 3;

        count += match (rem_threes, rem_twos) {
            (0, 0) => 0,
            (0, 1) | (0, 2) | (1, 0) | (1, 1) => 1, // one digit covers remainder
            (1, 2) => 2,                            // need '2' and '6'
            _ => unreachable!(),
        };
        count
    }

    /// Builds the result string from `prefix` bytes, a `changed_digit`, and an optimal suffix
    /// of length `suffix_len` that covers the remaining `factors`.
    #[inline]
    fn build_result(
        prefix: &[u8],
        changed_digit: u8,
        mut factors: PrimeFactorCount,
        suffix_len: usize,
    ) -> String {
        let mut result = Vec::with_capacity(prefix.len() + 1 + suffix_len);
        result.extend_from_slice(prefix);
        result.push(changed_digit + b'0');

        // Collect the required factor digits.
        let mut suffix = Vec::new();
        suffix.extend(vec![b'7'; factors.sevens as usize]);
        suffix.extend(vec![b'5'; factors.fives as usize]);

        while factors.threes >= 2 {
            suffix.push(b'9');
            factors.threes -= 2;
        }
        while factors.twos >= 3 {
            suffix.push(b'8');
            factors.twos -= 3;
        }

        match (factors.threes, factors.twos) {
            (0, 1) => suffix.push(b'2'),
            (0, 2) => suffix.push(b'4'),
            (1, 0) => suffix.push(b'3'),
            (1, 1) => suffix.push(b'6'),
            (1, 2) => {
                suffix.push(b'2');
                suffix.push(b'6');
            }
            _ => {}
        }

        // Sort required digits for lexicographic minimality, then pad with '1's.
        suffix.sort_unstable();
        let padding = suffix_len - suffix.len();
        result.extend(vec![b'1'; padding]);
        result.extend(suffix);

        // SAFETY: result contains only ASCII digits '0'-'9'.
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_basic_increment() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 256),
            "1488".to_string()
        );
    }

    #[test]
    fn test_example_already_valid() {
        assert_eq!(
            Solution::smallest_number("12355".to_string(), 50),
            "12355".to_string()
        );
    }

    #[test]
    fn test_example_impossible() {
        assert_eq!(
            Solution::smallest_number("11111".to_string(), 26),
            "-1".to_string()
        );
    }

    #[test]
    fn test_with_zero_in_num() {
        assert_eq!(
            Solution::smallest_number("102".to_string(), 2),
            "112".to_string()
        );
    }

    #[test]
    fn test_target_is_one() {
        assert_eq!(
            Solution::smallest_number("123".to_string(), 1),
            "123".to_string()
        );
    }

    #[test]
    fn test_needs_longer_number() {
        // "11", t=256=2^8; "488" (4×8×8=256) is the smallest valid answer.
        assert_eq!(
            Solution::smallest_number("11".to_string(), 256),
            "488".to_string()
        );
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(
            Solution::smallest_number("5".to_string(), 5),
            "5".to_string()
        );
    }

    #[test]
    fn test_large_target_small_num() {
        // t=1968750=2×3^2×5^6×7 needs 9 digits; "255555579" (product=1968750) is correct.
        assert_eq!(
            Solution::smallest_number("12".to_string(), 1_968_750),
            "255555579".to_string()
        );
    }

    #[test]
    fn test_zero_in_middle() {
        assert_eq!(
            Solution::smallest_number("4093".to_string(), 180),
            "4159".to_string()
        );
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(
            Solution::smallest_number("1111".to_string(), 7),
            "1117".to_string()
        );
    }

    #[test]
    fn test_prime_beyond_seven() {
        assert_eq!(
            Solution::smallest_number("999".to_string(), 11),
            "-1".to_string()
        );
    }

    #[test]
    fn test_num_with_zero_t320() {
        // Regression: num="10", t=320=2^6*5 caused capacity overflow in fallback path.
        // "588" (5×8×8=320) is the smallest 3-digit number with valid product.
        assert_eq!(
            Solution::smallest_number("10".to_string(), 320),
            "588".to_string()
        );
    }

    #[test]
    fn test_complex_factors() {
        // t=84=2^2×3×7; "267" (2×6×7=84) is smallest >= "123" with valid product.
        assert_eq!(
            Solution::smallest_number("123".to_string(), 84),
            "267".to_string()
        );
    }
}
