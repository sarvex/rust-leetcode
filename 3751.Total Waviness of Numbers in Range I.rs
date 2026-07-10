/// Per-digit transition state tracking how many numbers ending in a given digit
/// are rising (last step up), falling (last step down), or stagnant (last step flat),
/// along with the accumulated waviness contribution from those numbers.
#[derive(Clone, Copy, Default)]
struct State {
    rising: i32,
    falling: i32,
    stagnant: i32,
    waviness: i32,
}

impl State {
    /// Total count of numbers represented by this state.
    #[inline]
    fn total(self) -> i32 {
        self.rising + self.falling + self.stagnant
    }

    /// Extend all numbers tracked by `other` (which last transitioned via `other_digit`)
    /// by appending `digit` as the next digit, accumulating into `self`.
    ///
    /// A new peak/valley is created whenever the direction reverses:
    /// - appending a smaller digit after a rising sequence creates a peak (+rising)
    /// - appending a larger digit after a falling sequence creates a valley (+falling)
    #[inline]
    fn update(&mut self, digit: u8, other: State, other_digit: u8) {
        if digit < other_digit {
            self.falling += other.total();
            self.waviness += other.rising + other.waviness;
        } else if digit == other_digit {
            self.stagnant += other.total();
            self.waviness += other.waviness;
        } else {
            self.rising += other.total();
            self.waviness += other.falling + other.waviness;
        }
    }
}

impl Solution {
    /// Calculate total waviness for all numbers in range [num1, num2].
    ///
    /// # Intuition
    /// Track digit-DP states that record, for each possible last digit, how many
    /// numbers-so-far are rising/falling/stagnant at that digit, and how much
    /// waviness they have accumulated. Appending a new digit updates states in
    /// O(10²) per position — entirely independent of the range size.
    ///
    /// # Approach
    /// Reduce `[num1, num2]` to `f(num2) - f(num1 - 1)` where `f(n)` counts total
    /// waviness over `[1, n]`.
    ///
    /// `f(n)` processes the digits of `n` left-to-right, maintaining:
    /// - `states[d]`: aggregate state for all unconstrained numbers whose last digit is `d`
    /// - `limit`: aggregate state for the single number that is still "tight" (equal to the
    ///   prefix of `n` seen so far)
    ///
    /// At each position we:
    /// 1. Extend every free number by all 10 digits (updating `new_states`).
    /// 2. For digits strictly below the current digit of `n`, "release" the tight number
    ///    into the free pool.
    /// 3. Advance the tight number with exactly the current digit of `n`.
    ///
    /// # Complexity
    /// - Time: O(D × 10²) per call, D ≤ 6 for num2 ≤ 10^5 — effectively O(1)
    /// - Space: O(D) for digit extraction; O(10) for state arrays
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        if num1 > 0 {
            return Self::count_up_to(num2) - Self::count_up_to(num1 - 1);
        }
        Self::count_up_to(num2)
    }

    /// Total waviness of all integers in [1, n].
    fn count_up_to(n: i32) -> i32 {
        // Numbers with fewer than 3 digits have no interior positions → waviness 0.
        if n <= 99 {
            return 0;
        }

        // Extract digits of n, most-significant first.
        let mut digits = [0u8; 6];
        let mut len = 0usize;
        let mut tmp = n;
        while tmp > 0 {
            digits[len] = (tmp % 10) as u8;
            len += 1;
            tmp /= 10;
        }
        digits[..len].reverse();
        let digits = &digits[..len];

        // `limit` tracks the tight prefix (the number that matches n's digits so far).
        // Starts as a single stagnant "seed" before any digit has been appended.
        let mut limit = State {
            stagnant: 1,
            ..State::default()
        };
        let mut prior_limit_digit = digits[0];

        // `states[d]` = aggregate state for all free numbers whose last appended digit is d.
        // After processing the first digit of n, free numbers are those with a first digit
        // in [1, digits[0]-1] (no leading zeros for positive integers).
        let mut states = [State::default(); 10];
        for d in 1..prior_limit_digit {
            states[d as usize].stagnant = 1;
        }

        // Process each subsequent digit position.
        for &cur_limit_digit in &digits[1..] {
            // Step 1: extend every free number by all 10 possible next digits.
            let mut new_states = [State::default(); 10];
            for new_d in 0u8..10 {
                for old_d in 0u8..10 {
                    new_states[new_d as usize].update(new_d, states[old_d as usize], old_d);
                }
                // A digit > 0 can start a new free number of the current length.
                if new_d > 0 {
                    new_states[new_d as usize].stagnant += 1;
                }
            }

            // Step 2: release the tight prefix for all digits strictly below cur_limit_digit.
            for free_d in 0..cur_limit_digit {
                new_states[free_d as usize].update(free_d, limit, prior_limit_digit);
            }

            // Step 3: advance the tight number with cur_limit_digit.
            let mut new_limit = State::default();
            new_limit.update(cur_limit_digit, limit, prior_limit_digit);

            states = new_states;
            limit = new_limit;
            prior_limit_digit = cur_limit_digit;
        }

        // Sum waviness over all free numbers plus the tight number itself.
        let free_waviness: i32 = states.iter().map(|s| s.waviness).sum();
        free_waviness + limit.waviness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 120: 2 is peak (1<2>0) → 1; 121: 2 is peak → 1; 130: 3 is peak → 1
        assert_eq!(Solution::total_waviness(120, 130), 3);
    }

    #[test]
    fn test_example_2() {
        // 198→1, 199→0, 200→0, 201→1, 202→1
        assert_eq!(Solution::total_waviness(198, 202), 3);
    }

    #[test]
    fn test_example_3() {
        // 4848: 8 is peak, 4 is valley → 2
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }

    #[test]
    fn test_single_digit_range() {
        assert_eq!(Solution::total_waviness(1, 9), 0);
    }

    #[test]
    fn test_two_digit_range() {
        assert_eq!(Solution::total_waviness(10, 99), 0);
    }

    #[test]
    fn test_three_digit_valley() {
        // 101: 0 is valley (1>0<1) → 1
        assert_eq!(Solution::total_waviness(101, 101), 1);
    }

    #[test]
    fn test_three_digit_peak() {
        // 121: 2 is peak → 1
        assert_eq!(Solution::total_waviness(121, 121), 1);
    }

    #[test]
    fn test_multiple_peaks_valleys() {
        // 12121: peak@2, valley@1, peak@2 → 3
        assert_eq!(Solution::total_waviness(12121, 12121), 3);
    }

    #[test]
    fn test_equal_neighbors_no_waviness() {
        // 1221: no strict peak/valley → 0
        assert_eq!(Solution::total_waviness(1221, 1221), 0);
    }

    #[test]
    fn test_flat_number() {
        assert_eq!(Solution::total_waviness(100, 100), 0);
    }

    #[test]
    fn test_decade_boundary() {
        // 100: 0; 101–109: each has valley at tens digit → 9
        assert_eq!(Solution::total_waviness(100, 109), 9);
    }

    #[test]
    fn test_range_starting_at_one() {
        assert_eq!(Solution::total_waviness(1, 100), 0);
    }

    #[test]
    fn test_full_domain() {
        let result = Solution::total_waviness(1, 100_000);
        assert!(result > 0);
    }
}
