struct Solution;

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
    /// 1. Validate t contains only factors 2, 3, 5, 7. If not, return "-1"
    /// 2. Calculate prefix requirements, stopping at first zero (invalid digit)
    /// 3. If original number satisfies requirements, return it
    /// 4. Try incrementing each position from right to left, filling suffix optimally
    /// 5. Fallback to length n+1 if no same-length solution exists
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of num
    /// - Space: O(n) for storing prefix requirements and result
    pub fn smallest_number(num: String, target: i64) -> String {
        // Extract prime factors from target
        let required_factors = Self::extract_prime_factors(target);
        if required_factors.twos == u8::MAX {
            return "-1".to_string();
        }

        let input_bytes = num.as_bytes();
        let input_length = input_bytes.len();

        // Calculate minimum digits needed for target
        let min_digits_for_target = Self::compute_minimum_digits_needed(required_factors);

        // Early return if current number already satisfies and has no zeros
        if min_digits_for_target == 0 && !input_bytes.contains(&b'0') {
            return num;
        }

        // Build prefix factor requirements array
        let mut prefix_factors = Vec::with_capacity(input_length + 1);
        prefix_factors.push(required_factors);

        // Find first zero position
        let first_zero_pos = input_bytes
            .iter()
            .position(|&b| b == b'0')
            .unwrap_or(input_length);

        // Process each digit until zero, tracking remaining factors needed
        let remaining_factors =
            input_bytes[..first_zero_pos]
                .iter()
                .fold(required_factors, |factors, &digit_byte| {
                    let digit = digit_byte - b'0';
                    let new_factors = Self::subtract_digit_contribution(factors, digit);
                    prefix_factors.push(new_factors);
                    new_factors
                });

        // If no zeros and already satisfies, return original
        if first_zero_pos == input_length
            && Self::compute_minimum_digits_needed(remaining_factors) == 0
        {
            return num;
        }

        // Try incrementing from rightmost possible position
        let max_changeable_pos = first_zero_pos.min(input_length - 1);

        for change_pos in (0..=max_changeable_pos).rev() {
            let factors_before = prefix_factors[change_pos];
            let suffix_len = input_length - 1 - change_pos;
            let current_digit = input_bytes[change_pos] - b'0';

            // Try each larger digit at this position
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

        // No same-length solution found, create longer number
        let result_len = min_digits_for_target.max(input_length + 1);
        Self::build_result(&[], 1, required_factors, result_len - 1)
    }

    /// Extracts prime factors 2, 3, 5, 7 from target value.
    #[inline(always)]
    fn extract_prime_factors(mut value: i64) -> PrimeFactorCount {
        let mut factors = PrimeFactorCount::default();

        // Extract powers of 2
        while value % 2 == 0 {
            factors.twos += 1;
            value /= 2;
        }

        // Extract powers of 3
        while value % 3 == 0 {
            factors.threes += 1;
            value /= 3;
        }

        // Extract powers of 5
        while value % 5 == 0 {
            factors.fives += 1;
            value /= 5;
        }

        // Extract powers of 7
        while value % 7 == 0 {
            factors.sevens += 1;
            value /= 7;
        }

        // If any other prime factors remain, no solution exists
        if value != 1 {
            factors.twos = u8::MAX; // Sentinel value for invalid
        }

        factors
    }

    /// Subtracts contribution of a digit from remaining factor requirements.
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

    /// Calculates minimum digits needed to satisfy factor requirements.
    #[inline(always)]
    fn compute_minimum_digits_needed(factors: PrimeFactorCount) -> usize {
        // Each 7 needs digit 7, each 5 needs digit 5
        let mut count = factors.sevens as usize + factors.fives as usize;

        // Each pair of 3s can use digit 9, each triple of 2s can use digit 8
        count += (factors.threes / 2) as usize + (factors.twos / 3) as usize;

        // Handle remaining factors
        let rem_threes = factors.threes % 2;
        let rem_twos = factors.twos % 3;

        count += match (rem_threes, rem_twos) {
            (0, 0) => 0,                            // No remainder
            (0, 1) | (0, 2) | (1, 0) | (1, 1) => 1, // One digit handles remainder
            (1, 2) => 2,                            // Need two digits (2 and 6)
            _ => unreachable!(),
        };

        count
    }

    /// Builds the result string from prefix, changed digit, and optimal suffix.
    #[inline]
    fn build_result(
        prefix: &[u8],
        changed_digit: u8,
        mut factors: PrimeFactorCount,
        suffix_len: usize,
    ) -> String {
        let mut result = Vec::with_capacity(prefix.len() + 1 + suffix_len);

        // Add prefix and changed digit
        result.extend_from_slice(prefix);
        result.push(changed_digit + b'0');

        // Build optimal suffix digits
        let mut suffix = Vec::with_capacity(suffix_len);

        // Add required prime factor digits
        suffix.extend(vec![b'7'; factors.sevens as usize]);
        suffix.extend(vec![b'5'; factors.fives as usize]);

        // Use 9s for pairs of 3s
        while factors.threes >= 2 {
            suffix.push(b'9');
            factors.threes -= 2;
        }

        // Use 8s for triples of 2s
        while factors.twos >= 3 {
            suffix.push(b'8');
            factors.twos -= 3;
        }

        // Handle remaining factors with appropriate digits
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

        // Sort suffix digits to get smallest lexicographic result
        suffix.sort_unstable();

        // Pad with 1s if needed
        let padding = suffix_len - suffix.len();
        result.extend(vec![b'1'; padding]);
        result.extend(suffix);

        // SAFETY: result contains only ASCII digits '0'-'9', which are valid UTF-8
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
        assert_eq!(
            Solution::smallest_number("11".to_string(), 256),
            "188".to_string()
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
        assert_eq!(
            Solution::smallest_number("12".to_string(), 1968750),
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
    fn test_complex_factors() {
        assert_eq!(
            Solution::smallest_number("123".to_string(), 84),
            "136".to_string()
        );
    }
}
