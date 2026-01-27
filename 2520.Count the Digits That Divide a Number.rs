impl Solution {
    /// Counts digits of num that evenly divide num.
    ///
    /// # Intuition
    /// Extract each digit and check divisibility. Every digit in the input
    /// is guaranteed to be non-zero.
    ///
    /// # Approach
    /// Repeatedly extract the last digit via modulo, check if num is divisible by it.
    ///
    /// # Complexity
    /// - Time: O(log num) — number of digits
    /// - Space: O(1)
    pub fn count_digits(num: i32) -> i32 {
        let mut cur = num;
        let mut count = 0;
        while cur > 0 {
            if num % (cur % 10) == 0 {
                count += 1;
            }
            cur /= 10;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_divide() {
        assert_eq!(Solution::count_digits(1248), 4);
    }

    #[test]
    fn test_partial_divide() {
        assert_eq!(Solution::count_digits(7), 1);
    }

    #[test]
    fn test_repeated_digit() {
        assert_eq!(Solution::count_digits(121), 2);
    }
}
