impl Solution {
    /// Digit-by-digit reversal with overflow detection.
    ///
    /// # Intuition
    /// Extract digits from the least significant end and accumulate them
    /// into a reversed number. Rust's `checked_mul` and `checked_add`
    /// detect 32-bit overflow without relying on string conversion.
    ///
    /// # Approach
    /// Repeatedly extract the last digit via modulo and remove it via
    /// division. Build the reversed value using checked arithmetic,
    /// returning 0 on overflow.
    ///
    /// # Complexity
    /// - Time: O(log x) — proportional to the number of digits
    /// - Space: O(1) — scalar variables only
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut reversed = 0i32;

        while num != 0 {
            let digit = num % 10;
            num /= 10;
            reversed = match reversed.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                Some(val) => val,
                None => return 0,
            };
        }

        reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_number() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn negative_number() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn trailing_zeros() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn overflow_positive() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn overflow_negative() {
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
