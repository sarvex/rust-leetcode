impl Solution {
    /// Concatenates non-zero digits and multiplies by their sum.
    ///
    /// # Intuition
    /// Extract all non-zero digits from the number, concatenate them to form a new number x,
    /// calculate the sum of digits in x, and return x * sum.
    ///
    /// # Approach
    /// 1. Extract digits from n by converting to string
    /// 2. Filter out zeros and collect non-zero digits
    /// 3. Concatenate non-zero digits to form x
    /// 4. Calculate sum of digits in x
    /// 5. Return x * sum
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits in n (at most 10 for n <= 10^9)
    /// - Space: O(d) for storing the digits
    pub fn sum_and_multiply(n: i32) -> i64 {
        // Extract non-zero digits
        let digits: Vec<u8> = n.to_string().bytes().filter(|&b| b != b'0').collect();

        // If no non-zero digits, x = 0
        if digits.is_empty() {
            return 0;
        }

        // Form x by concatenating non-zero digits
        let x: i64 = digits
            .iter()
            .fold(0i64, |acc, &d| acc * 10 + (d - b'0') as i64);

        // Calculate sum of digits in x
        let sum: i64 = digits.iter().map(|&d| (d - b'0') as i64).sum();

        // Return x * sum
        x * sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::sum_and_multiply(10203004), 12340);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::sum_and_multiply(1000), 1);
    }

    #[test]
    fn test_edge_zero() {
        assert_eq!(Solution::sum_and_multiply(0), 0);
    }

    #[test]
    fn test_edge_single_digit() {
        assert_eq!(Solution::sum_and_multiply(5), 25); // x = 5, sum = 5, result = 25
    }

    #[test]
    fn test_all_zeros_except_one() {
        assert_eq!(Solution::sum_and_multiply(100000), 1); // x = 1, sum = 1
    }

    #[test]
    fn test_no_zeros() {
        assert_eq!(Solution::sum_and_multiply(123), 738); // x = 123, sum = 6, result = 738
    }

    #[test]
    fn test_boundary_max() {
        assert_eq!(Solution::sum_and_multiply(999999999), 80999999919); // x = 999999999, sum = 81
    }

    #[test]
    fn test_alternating_zeros() {
        // non-zero digits are 1, 2, 3 → x = 123, sum = 6, result = 738
        assert_eq!(Solution::sum_and_multiply(102030), 738);
    }
}
