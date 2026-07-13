impl Solution {
    /// Generate all sequential digit numbers in [low, high] using sliding window over "123456789".
    ///
    /// # Intuition
    /// All sequential digit numbers are substrings of "123456789". We enumerate every
    /// contiguous substring of length 1–9 and collect those that fall within [low, high].
    /// Because we iterate by increasing length and left-to-right within each length,
    /// results are naturally sorted.
    ///
    /// # Approach
    /// 1. Treat "123456789" as the source of all possible sequential digit numbers.
    /// 2. For each window length `len` from 1 to 9, slide across the 9 digits.
    /// 3. Parse each window as an integer and include it if it lies in [low, high].
    ///
    /// # Complexity
    /// - Time: O(1) — at most 45 windows (9+8+…+1) regardless of input
    /// - Space: O(1) — output aside, no auxiliary storage
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let digits = b"123456789";
        let mut result = Vec::new();

        for len in 1..=9_usize {
            for start in 0..=(9 - len) {
                let num = digits[start..start + len]
                    .iter()
                    .fold(0i32, |acc, &d| acc * 10 + (d - b'0') as i32);
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }

    #[test]
    fn test_single_digit_range() {
        assert_eq!(
            Solution::sequential_digits(10, 99),
            vec![12, 23, 34, 45, 56, 67, 78, 89]
        );
    }

    #[test]
    fn test_full_range() {
        let result = Solution::sequential_digits(10, 1_000_000_000);
        // 2+3+4+5+6+7+8+9 windows of length 2..9, plus 1 of length 1 (1..9 digits)
        // Total: 8+7+6+5+4+3+2+1 = 36 numbers (length 2–9) + 9 (length 1) = 45 minus those < 10
        assert_eq!(result.len(), 36); // lengths 2–9: 8+7+6+5+4+3+2+1 = 36
        assert_eq!(*result.first().unwrap(), 12);
        assert_eq!(*result.last().unwrap(), 123_456_789);
    }

    #[test]
    fn test_exact_boundary() {
        assert_eq!(Solution::sequential_digits(123, 123), vec![123]);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(Solution::sequential_digits(200, 220), vec![]);
    }
}
