impl Solution {
    /// Checks if num equals some non-negative integer plus its digit-reversal.
    ///
    /// # Intuition
    /// Since num <= 10^5, we only need to check candidates from 0 to num.
    /// For each candidate, compute its reverse and check if the sum matches.
    ///
    /// # Approach
    /// 1. Iterate i from 0 to num
    /// 2. Compute the digit reversal of i
    /// 3. Return true if i + reverse(i) == num
    ///
    /// # Complexity
    /// - Time: O(n * d) where d is digit count
    /// - Space: O(1)
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        let reverse_digits = |mut n: i32| -> i32 {
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            rev
        };

        (0..=num).any(|i| i + reverse_digits(i) == num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::sum_of_number_and_reverse(443));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::sum_of_number_and_reverse(63));
    }

    #[test]
    fn test_zero() {
        assert!(Solution::sum_of_number_and_reverse(0));
    }

    #[test]
    fn test_palindrome_sum() {
        assert!(Solution::sum_of_number_and_reverse(2));
    }

    #[test]
    fn test_one() {
        assert!(!Solution::sum_of_number_and_reverse(1));
    }
}
