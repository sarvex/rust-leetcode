impl Solution {
    /// Finds the second largest digit in a string.
    ///
    /// # Intuition
    /// Track the two largest distinct digits while scanning. Each digit
    /// either becomes the new largest (pushing the old largest to second)
    /// or updates the second largest directly.
    ///
    /// # Approach
    /// 1. Iterate over bytes, filtering for ASCII digits.
    /// 2. Maintain `first` and `second` as the two largest distinct digits.
    /// 3. Update them as each digit is encountered.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn second_highest(s: String) -> i32 {
        let (mut first, mut second) = (-1, -1);

        for &b in s.as_bytes() {
            if b.is_ascii_digit() {
                let num = (b - b'0') as i32;
                if num > first {
                    second = first;
                    first = num;
                } else if num < first && num > second {
                    second = num;
                }
            }
        }

        second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_string() {
        assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    }

    #[test]
    fn test_no_digits() {
        assert_eq!(Solution::second_highest("abc".to_string()), -1);
    }

    #[test]
    fn test_two_distinct_digits() {
        assert_eq!(Solution::second_highest("a9b8".to_string()), 8);
    }
}
