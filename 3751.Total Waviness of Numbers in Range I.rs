impl Solution {
    /// Calculate total waviness for all numbers in range [num1, num2].
    ///
    /// # Intuition
    /// Waviness is the count of peaks and valleys in a number's digits.
    /// For the given constraint (num2 <= 10^5, max 100k numbers), a simple
    /// iterative approach is optimal - no HashMap overhead, no recursion.
    ///
    /// # Approach
    /// For each number in [num1, num2]:
    /// 1. Extract digits using division/modulo (faster than to_string)
    /// 2. Use fixed-size array of 6 elements (max digits for 10^5) instead of Vec
    /// 3. Check each interior digit for peak (greater than both neighbors) or
    ///    valley (less than both neighbors)
    /// 4. Sum counts using iterator fold
    ///
    /// # Complexity
    /// - Time: O((num2 - num1 + 1) * d) where d ≤ 6 (at most 600k operations)
    /// - Space: O(1) - fixed size array on stack
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (num1..=num2)
            .map(Self::calculate_waviness)
            .sum()
    }

    /// Calculate waviness of a single number using stack-allocated digit array.
    #[inline]
    fn calculate_waviness(mut num: i32) -> i32 {
        // Fixed-size array for digits (10^5 has at most 6 digits)
        let mut digits = [0i32; 6];
        let mut len = 0usize;

        // Extract digits (LSB first, then reverse mentally by reading backwards)
        while num > 0 {
            digits[len] = num % 10;
            num /= 10;
            len += 1;
        }

        // Numbers with fewer than 3 digits have waviness 0
        if len < 3 {
            return 0;
        }

        // Check interior positions for peaks/valleys
        // digits are stored LSB-first, so we check from len-2 down to 1
        // position i in original = digits[len-1-i] in our array
        // We want to check positions 1 to len-2 in original number
        // Original: d[len-1] ... d[2] d[1] d[0]
        // Check: d[1] is middle of (d[2], d[1], d[0])
        // In our array: digits[1] is d[1], digits[0] is d[0], digits[2] is d[2]
        // Wait, let's think again:
        // num = 123, we extract: digits[0]=3, digits[1]=2, digits[2]=1, len=3
        // Original number: digits[2]digits[1]digits[0] = 123
        // Check position 1 (digit 2): neighbors are digits[2]=1 and digits[0]=3
        // So we check: digits[1] against digits[2] and digits[0]

        let mut waviness = 0;
        for mid in 1..len - 1 {
            let curr = digits[mid];
            let left = digits[mid + 1]; // higher digit (more significant)
            let right = digits[mid - 1]; // lower digit (less significant)

            if (curr > left && curr > right) || (curr < left && curr < right) {
                waviness += 1;
            }
        }

        waviness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 120: 2 is peak (1 < 2 > 0) -> 1
        // 121: 2 is peak (1 < 2 > 1) -> 1
        // 130: 3 is peak (1 < 3 > 0) -> 1
        assert_eq!(Solution::total_waviness(120, 130), 3);
    }

    #[test]
    fn test_example_2() {
        // 198: 9 is peak (1 < 9 > 8) -> 1
        // 199: no peak/valley -> 0
        // 200: no peak/valley -> 0
        // 201: 0 is valley (2 > 0 < 1) -> 1
        // 202: 0 is valley (2 > 0 < 2) -> 1
        assert_eq!(Solution::total_waviness(198, 202), 3);
    }

    #[test]
    fn test_example_3() {
        // 4848: 8 is peak (4 < 8 > 4), 4 is valley (8 > 4 < 8) -> 2
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }

    #[test]
    fn test_single_digit_numbers() {
        assert_eq!(Solution::total_waviness(1, 9), 0);
    }

    #[test]
    fn test_two_digit_numbers() {
        assert_eq!(Solution::total_waviness(10, 99), 0);
    }

    #[test]
    fn test_three_digit_valley() {
        // 101: 0 is valley (1 > 0 < 1) -> 1
        assert_eq!(Solution::total_waviness(101, 101), 1);
    }

    #[test]
    fn test_three_digit_peak() {
        // 121: 2 is peak (1 < 2 > 1) -> 1
        assert_eq!(Solution::total_waviness(121, 121), 1);
    }

    #[test]
    fn test_multiple_peaks_valleys() {
        // 12121: digits [1,2,1,2,1]
        // windows: [1,2,1] -> 2 is peak, [2,1,2] -> 1 is valley, [1,2,1] -> 2 is peak
        assert_eq!(Solution::total_waviness(12121, 12121), 3);
    }

    #[test]
    fn test_equal_neighbors() {
        // 1221: 2 == 2, so no peak or valley
        assert_eq!(Solution::total_waviness(1221, 1221), 0);
    }

    #[test]
    fn test_range_of_one() {
        assert_eq!(Solution::total_waviness(100, 100), 0);
    }

    #[test]
    fn test_large_range_small_numbers() {
        // 100-109: 100 has no waviness (0 is not < 0)
        // 101-109: 0 is valley in all (1 > 0 < last_digit where last_digit >= 1)
        // Total: 9 numbers with waviness 1 each = 9
        assert_eq!(Solution::total_waviness(100, 109), 9);
    }
}
