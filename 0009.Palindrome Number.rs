impl Solution {
    /// Half-reversal comparison for palindrome number detection.
    ///
    /// # Intuition
    /// A palindrome reads the same forwards and backwards. Reversing only the
    /// second half and comparing it to the first half avoids overflow and
    /// processes only half the digits.
    ///
    /// # Approach
    /// Reject negatives and multiples of 10 (except 0). Repeatedly extract
    /// the last digit of `x` and build a reversed half. Stop when the
    /// reversed half is at least as large as the remaining `x`. Compare
    /// directly for even-length numbers, or after dropping the middle digit
    /// for odd-length numbers.
    ///
    /// # Complexity
    /// - Time: O(log x) — processes half the digits
    /// - Space: O(1) — scalar variables only
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }

        let mut reversed_half = 0;
        while x > reversed_half {
            reversed_half = reversed_half * 10 + x % 10;
            x /= 10;
        }

        x == reversed_half || x == reversed_half / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_digit_palindrome() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn negative_number() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn trailing_zero() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn zero() {
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn even_digit_palindrome() {
        assert!(Solution::is_palindrome(1221));
    }

    #[test]
    fn single_digit() {
        assert!(Solution::is_palindrome(7));
    }
}
