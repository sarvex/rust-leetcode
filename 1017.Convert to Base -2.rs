pub struct Solution;

impl Solution {
    /// Converts an integer to base -2 representation.
    ///
    /// # Intuition
    /// Division by -2 works similarly to normal base conversion, but the
    /// remainder must be non-negative. Adjust the quotient when the
    /// remainder is negative.
    ///
    /// # Approach
    /// Repeatedly divide by 2 while alternating the sign factor. If the
    /// current bit is set, subtract the sign factor and push '1'; otherwise
    /// push '0'. Reverse the collected digits.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(log n) for the result string
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut bits = Vec::new();
        let mut sign = 1;
        while n != 0 {
            if n % 2 != 0 {
                bits.push(b'1');
                n -= sign;
            } else {
                bits.push(b'0');
            }
            sign *= -1;
            n /= 2;
        }
        bits.reverse();
        String::from_utf8(bits).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!(Solution::base_neg2(2), "110");
    }

    #[test]
    fn test_three() {
        assert_eq!(Solution::base_neg2(3), "111");
    }

    #[test]
    fn test_four() {
        assert_eq!(Solution::base_neg2(4), "100");
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::base_neg2(0), "0");
    }
}
