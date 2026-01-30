impl Solution {
    /// Binary exponentiation (fast power) for computing x^n.
    ///
    /// # Intuition
    /// Exponentiation by squaring reduces the number of multiplications
    /// from O(n) to O(log n) by halving the exponent at each step and
    /// squaring the base.
    ///
    /// # Approach
    /// Convert `n` to its unsigned absolute value to handle `i32::MIN`.
    /// If `n` is negative, invert the base once. Iteratively square the
    /// base and multiply into the result when the current bit of the
    /// exponent is set.
    ///
    /// # Complexity
    /// - Time: O(log n) — halving the exponent each iteration
    /// - Space: O(1) — scalar variables only
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut base = x;
        let mut exp = n.unsigned_abs() as u64;
        if n < 0 {
            base = 1.0 / base;
        }

        let mut result = 1.0;
        while exp != 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base *= base;
            exp >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_exponent() {
        assert!((Solution::my_pow(2.0, 10) - 1024.0).abs() < 1e-5);
    }

    #[test]
    fn negative_exponent() {
        assert!((Solution::my_pow(2.0, -2) - 0.25).abs() < 1e-5);
    }

    #[test]
    fn zero_exponent() {
        assert!((Solution::my_pow(2.0, 0) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn fractional_base() {
        assert!((Solution::my_pow(2.1, 3) - 9.261).abs() < 1e-3);
    }

    #[test]
    fn min_exponent() {
        assert!((Solution::my_pow(1.0, i32::MIN) - 1.0).abs() < 1e-5);
    }
}
