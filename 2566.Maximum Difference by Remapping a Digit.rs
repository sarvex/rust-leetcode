impl Solution {
    /// Maximize difference by remapping one digit to 0 and another to 9.
    ///
    /// # Intuition
    /// To minimize, remap the leading digit to 0. To maximize, remap the first
    /// non-9 digit to 9. The difference of these gives the answer.
    ///
    /// # Approach
    /// 1. Convert num to string
    /// 2. For min: replace all occurrences of the first digit with '0'
    /// 3. For max: find the first non-9 digit and replace all its occurrences with '9'
    /// 4. Return max - min
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits
    /// - Space: O(d)
    pub fn min_max_difference(num: i32) -> i32 {
        let s = num.to_string();
        let bytes = s.as_bytes();
        let min_val: i32 = s.replace(char::from(bytes[0]), "0").parse().unwrap_or(0);

        let max_val: i32 = bytes
            .iter()
            .find(|&&c| c != b'9')
            .map(|&c| s.replace(char::from(c), "9").parse().unwrap_or(num))
            .unwrap_or(num);

        max_val - min_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::min_max_difference(11891), 99009);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::min_max_difference(5), 9);
    }

    #[test]
    fn test_all_nines() {
        assert_eq!(Solution::min_max_difference(999), 999);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::min_max_difference(90), 99);
    }
}
