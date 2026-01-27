impl Solution {
    /// Maximum digit determines minimum deci-binary partition count.
    ///
    /// # Intuition
    /// Each deci-binary number contributes at most 1 to each digit position.
    /// The digit with the highest value determines the minimum numbers needed.
    ///
    /// # Approach
    /// 1. Find the maximum digit in the string
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().map(|b| (b - b'0') as i32).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_digit_three() {
        assert_eq!(Solution::min_partitions("32".to_string()), 3);
    }

    #[test]
    fn max_digit_eight() {
        assert_eq!(Solution::min_partitions("82734".to_string()), 8);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::min_partitions("11111".to_string()), 1);
    }
}
