impl Solution {
    /// Replace the first '6' digit with '9' for maximum value.
    ///
    /// # Intuition
    /// Changing the leftmost '6' to '9' yields the largest possible number
    /// since higher-order digits contribute more to the total value.
    ///
    /// # Approach
    /// 1. Convert number to string
    /// 2. Replace the first occurrence of '6' with '9'
    /// 3. Parse back to integer
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits
    /// - Space: O(d) for the string representation
    pub fn maximum69_number(num: i32) -> i32 {
        num.to_string().replacen('6', "9", 1).parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_six() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
    }

    #[test]
    fn leading_six() {
        assert_eq!(Solution::maximum69_number(6699), 9699);
    }

    #[test]
    fn all_nines() {
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }

    #[test]
    fn single_digit() {
        assert_eq!(Solution::maximum69_number(6), 9);
    }
}
