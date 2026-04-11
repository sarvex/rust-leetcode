impl Solution {
    /// Separate each integer into its individual digits.
    ///
    /// # Intuition
    /// Extract digits from each number using arithmetic division and modulo,
    /// avoiding string allocation entirely.
    ///
    /// # Approach
    /// For each number, repeatedly extract the last digit and collect in reverse,
    /// then extend the result.
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is the average number of digits
    /// - Space: O(n * d)
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for &num in &nums {
            let start = result.len();
            let mut n = num;
            if n == 0 {
                result.push(0);
                continue;
            }
            while n > 0 {
                result.push(n % 10);
                n /= 10;
            }
            result[start..].reverse();
        }
        result
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
