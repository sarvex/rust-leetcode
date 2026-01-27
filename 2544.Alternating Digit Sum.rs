impl Solution {
    /// Computes the alternating sum of digits from most significant to least.
    ///
    /// # Intuition
    /// Processing digits from least significant and alternating signs, the final
    /// sign depends on parity. Multiplying by -sign at the end corrects the order.
    ///
    /// # Approach
    /// Extract digits right-to-left, alternating +/-, then negate if the total
    /// digit count is even (since we started from the wrong end).
    ///
    /// # Complexity
    /// - Time: O(log n) — number of digits
    /// - Space: O(1)
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut sign = 1;
        while n > 0 {
            sum += (n % 10) * sign;
            sign = -sign;
            n /= 10;
        }
        sum * -sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::alternate_digit_sum(111), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::alternate_digit_sum(7), 7);
    }
}
