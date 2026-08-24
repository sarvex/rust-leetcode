impl Solution {
    /// Checks if n is divisible by the sum of its digit sum and digit product.
    ///
    /// # Intuition
    /// Extract each digit to compute both the digit sum and digit product simultaneously,
    /// then check if n is divisible by their sum.
    ///
    /// # Approach
    /// 1. Iterate through each digit of n by repeatedly taking `n % 10` and dividing by 10.
    /// 2. Accumulate the digit sum and digit product.
    /// 3. Return whether `n % (digit_sum + digit_product) == 0`.
    ///
    /// # Complexity
    /// - Time: O(d) where d is the number of digits in n
    /// - Space: O(1)
    pub fn check_divisibility(n: i32) -> bool {
        let mut num = n;
        let mut digit_sum = 0;
        let mut digit_product = 1;

        while num > 0 {
            let d = num % 10;
            digit_sum += d;
            digit_product *= d;
            num /= 10;
        }

        let total = digit_sum + digit_product;
        total != 0 && n % total == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::check_divisibility(99));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::check_divisibility(23));
    }

    #[test]
    fn test_single_digit() {
        // n=5: digit_sum=5, digit_product=5, total=10, 5%10 != 0
        assert!(!Solution::check_divisibility(5));
        // n=2: digit_sum=2, digit_product=2, total=4, 2%4 != 0
        assert!(!Solution::check_divisibility(2));
    }

    #[test]
    fn test_contains_zero() {
        // n=10: digit_sum=1, digit_product=0, total=1, 10%1 == 0
        assert!(Solution::check_divisibility(10));
    }

    #[test]
    fn test_large_value() {
        // n=1000000: digit_sum=1, digit_product=0, total=1, 1000000%1 == 0
        assert!(Solution::check_divisibility(1_000_000));
    }
}
