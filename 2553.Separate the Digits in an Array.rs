impl Solution {
    /// Separate each integer into its individual digits.
    ///
    /// # Intuition
    /// Convert each number to its string representation and collect individual digits.
    ///
    /// # Approach
    /// Use `flat_map` to iterate over each number's string digits, parsing each character
    /// back to an i32.
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the average number of digits
    /// - Space: O(n * d)
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .flat_map(|&num| {
                num.to_string()
                    .bytes()
                    .map(|b| i32::from(b - b'0'))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_digits() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
    }

    #[test]
    fn test_large_and_small() {
        assert_eq!(
            Solution::separate_digits(vec![7, 1, 3, 9]),
            vec![7, 1, 3, 9]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::separate_digits(vec![100]), vec![1, 0, 0]);
    }
}
