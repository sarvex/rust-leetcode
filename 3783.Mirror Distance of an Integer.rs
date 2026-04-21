impl Solution {
    /// Computes the mirror distance `|n - reverse(n)|` in constant time.
    ///
    /// # Intuition
    /// The reverse of a 32-bit-bounded integer (`n <= 10^9`) fits comfortably in
    /// `i64`, so we can reverse digit-by-digit without worrying about overflow
    /// and then take the absolute difference.
    ///
    /// # Approach
    /// Iterate on a mutable copy of `n`, peeling off the least significant digit
    /// with `% 10` and accumulating it into `reversed` via `reversed * 10 + d`.
    /// Because the accumulator is `i64`, even the largest input
    /// (`n = 10^9`, reversed = `9_999_999_990` in the worst shape of 10-digit
    /// numbers, but here capped by the constraint) stays well within range.
    /// Finally, return `(n as i64 - reversed).abs() as i32`.
    ///
    /// # Complexity
    /// - Time: O(log10 n) — one step per digit.
    /// - Space: O(1).
    pub fn mirror_distance(n: i32) -> i32 {
        let mut x = n as i64;
        let mut reversed: i64 = 0;
        while x > 0 {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }
        (n as i64 - reversed).unsigned_abs() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::mirror_distance(25), 27);
    }

    #[test]
    fn test_example_two_trailing_zero() {
        assert_eq!(Solution::mirror_distance(10), 9);
    }

    #[test]
    fn test_example_three_single_digit() {
        assert_eq!(Solution::mirror_distance(7), 0);
    }

    #[test]
    fn test_palindrome_multi_digit() {
        assert_eq!(Solution::mirror_distance(121), 0);
        assert_eq!(Solution::mirror_distance(1221), 0);
    }

    #[test]
    fn test_min_boundary() {
        assert_eq!(Solution::mirror_distance(1), 0);
    }

    #[test]
    fn test_trailing_zeros_collapse() {
        // reverse(100) = 001 = 1 -> |100 - 1| = 99
        assert_eq!(Solution::mirror_distance(100), 99);
        // reverse(1000) = 1 -> 999
        assert_eq!(Solution::mirror_distance(1000), 999);
    }

    #[test]
    fn test_max_constraint() {
        // n = 10^9 -> reverse = 1 -> diff = 999_999_999
        assert_eq!(Solution::mirror_distance(1_000_000_000), 999_999_999);
    }

    #[test]
    fn test_generic_multi_digit() {
        // reverse(1234) = 4321 -> |1234 - 4321| = 3087
        assert_eq!(Solution::mirror_distance(1234), 3087);
        // reverse(90) = 9 -> 81
        assert_eq!(Solution::mirror_distance(90), 81);
    }

    #[test]
    fn test_near_palindrome() {
        // reverse(123) = 321 -> 198
        assert_eq!(Solution::mirror_distance(123), 198);
    }
}
