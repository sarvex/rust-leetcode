impl Solution {
    /// Converts an integer to its base-7 string representation.
    ///
    /// # Intuition
    /// Repeatedly divide by 7 and collect remainders, handling the sign separately.
    ///
    /// # Approach
    /// 1. Handle zero as a special case.
    /// 2. Work with the absolute value, collecting remainders.
    /// 3. Reverse the collected digits and prepend '-' if negative.
    ///
    /// # Complexity
    /// - Time: O(log₇ n)
    /// - Space: O(log₇ n)
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let negative = num < 0;
        let mut n = num.abs();
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((b'0' + (n % 7) as u8) as char);
            n /= 7;
        }
        if negative {
            digits.push('-');
        }
        digits.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        assert_eq!(Solution::convert_to_base7(100), "202");
    }

    #[test]
    fn test_negative() {
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::convert_to_base7(0), "0");
    }
}
