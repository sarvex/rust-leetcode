/// Tracks required counts of prime factors 2, 3, 5, 7 for divisibility.
#[derive(Clone, Copy)]
struct PrimeFactorCount {
    twos: u8,
    threes: u8,
    fives: u8,
    sevens: u8,
}

/// Precomputed factor contributions for digits 0-9: (twos, threes, fives, sevens)
const DIGIT_FACTORS: [(u8, u8, u8, u8); 10] = [
    (0, 0, 0, 0), // 0
    (0, 0, 0, 0), // 1
    (1, 0, 0, 0), // 2
    (0, 1, 0, 0), // 3
    (2, 0, 0, 0), // 4
    (0, 0, 1, 0), // 5
    (1, 1, 0, 0), // 6
    (0, 0, 0, 1), // 7
    (3, 0, 0, 0), // 8
    (0, 2, 0, 0), // 9
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
    /// 2. Calculate prefix requirements, stopping at first zero (invalid digit).
    /// 3. If original number satisfies requirements, return it.
    /// 4. Try incrementing each position from right to left, filling suffix optimally.
    /// 5. Fallback to length n+1 if no same-length solution exists.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of num
    /// - Space: O(n) for storing prefix requirements and result
    pub fn smallest_number(num: String, t: i64) -> String {
        let required_factors = Self::extract_prime_factors(t);
        if required_factors.twos == u8::MAX {
            return "-1".into();
        }

        let bytes = num.as_bytes();
        let num_length = bytes.len();

        let min_digits_for_t = Self::min_digits_needed(required_factors);

        // Early check: if t=1, any zero-free number works
        if min_digits_for_t == 0 {
            if !bytes.contains(&b'0') {
                return num;
            }
        }

        let mut prefix_requirements = Vec::with_capacity(num_length + 1);
        prefix_requirements.push(required_factors);

        let mut remaining_factors = required_factors;
        let mut first_zero_index = num_length;

        for (idx, &byte) in bytes.iter().enumerate() {
            if byte == b'0' {
                first_zero_index = idx;
                break;
            }
            remaining_factors = Self::subtract_digit_factors(remaining_factors, byte - b'0');
            prefix_requirements.push(remaining_factors);
        }

        // Check if original number is valid
        if first_zero_index == num_length && Self::min_digits_needed(remaining_factors) == 0 {
            return num;
        }

        let max_changeable_index = first_zero_index.min(num_length - 1);

        // Try incrementing each position from right to left
        for change_index in (0..=max_changeable_index).rev() {
            let factors_before = prefix_requirements[change_index];
            let suffix_length = num_length - 1 - change_index;
            let current_digit = bytes[change_index] - b'0';

            for candidate in (current_digit + 1)..=9 {
                let factors_after = Self::subtract_digit_factors(factors_before, candidate);
                let min_needed = Self::min_digits_needed(factors_after);

                if min_needed <= suffix_length {
                    return Self::build_result(
                        &bytes[..change_index],
                        candidate,
                        factors_after,
                        suffix_length,
                    );
                }
            }
        }

        // Fallback: smallest number with more digits
        let result_length = min_digits_for_t.max(num_length + 1);
        Self::build_result(&[], b'1' - b'0', required_factors, result_length - 1)
    }

    #[inline(always)]
    fn extract_prime_factors(mut target: i64) -> PrimeFactorCount {
        let mut twos = 0u8;
        while target % 2 == 0 {
            twos += 1;
            target /= 2;
        }
        let mut threes = 0u8;
        while target % 3 == 0 {
            threes += 1;
            target /= 3;
        }
        let mut fives = 0u8;
        while target % 5 == 0 {
            fives += 1;
            target /= 5;
        }
        let mut sevens = 0u8;
        while target % 7 == 0 {
            sevens += 1;
            target /= 7;
        }
        if target != 1 {
            return PrimeFactorCount {
                twos: u8::MAX,
                threes: 0,
                fives: 0,
                sevens: 0,
            };
        }
        PrimeFactorCount {
            twos,
            threes,
            fives,
            sevens,
        }
    }

    #[inline(always)]
    fn subtract_digit_factors(factors: PrimeFactorCount, digit: u8) -> PrimeFactorCount {
        let (s2, s3, s5, s7) = DIGIT_FACTORS[digit as usize];
        PrimeFactorCount {
            twos: factors.twos.saturating_sub(s2),
            threes: factors.threes.saturating_sub(s3),
            fives: factors.fives.saturating_sub(s5),
            sevens: factors.sevens.saturating_sub(s7),
        }
    }

    /// Calculates minimum digits needed to satisfy factor requirements.
    #[inline(always)]
    fn min_digits_needed(factors: PrimeFactorCount) -> usize {
        let mut count = factors.sevens as usize + factors.fives as usize;

        let threes = factors.threes as usize;
        let twos = factors.twos as usize;

        // Use 9s for pairs of 3s, 8s for triples of 2s
        count += threes / 2 + twos / 3;

        let rem3 = threes % 2;
        let rem2 = twos % 3;

        // Handle remainders optimally
        count += match (rem3, rem2) {
            (0, 0) => 0,
            (0, 1) | (0, 2) | (1, 0) | (1, 1) => 1,
            (1, 2) => 2,
            _ => 0,
        };

        count
    }

    /// Builds the result string from prefix, changed digit, and optimal suffix.
    #[inline]
    fn build_result(
        prefix: &[u8],
        changed_digit: u8,
        mut factors: PrimeFactorCount,
        suffix_length: usize,
    ) -> String {
        let total_length = prefix.len() + 1 + suffix_length;
        let mut result = Vec::with_capacity(total_length);

        result.extend_from_slice(prefix);
        result.push(changed_digit + b'0');

        // Build suffix digits
        let mut suffix = Vec::with_capacity(suffix_length);

        for _ in 0..factors.sevens {
            suffix.push(b'7');
        }
        for _ in 0..factors.fives {
            suffix.push(b'5');
        }
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

        suffix.sort_unstable();

        // Pad with 1s, then append sorted suffix
        let padding = suffix_length - suffix.len();
        result.resize(result.len() + padding, b'1');
        result.extend(suffix);

        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 256),
            "1488".to_string()
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::smallest_number("12355".to_string(), 50),
            "12355".to_string()
        );
    }

    #[test]
    fn test_example3() {
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
    fn test_t_is_one() {
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
    fn test_large_t_small_num() {
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
}
