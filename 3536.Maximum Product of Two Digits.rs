impl Solution {
    /// Find maximum product of any two digits in n using a single pass for top-two.
    ///
    /// # Intuition
    /// The maximum product of two digits is always the product of the two largest
    /// digits. Extract digits by repeatedly taking modulo 10, track the top two
    /// values, and multiply them.
    ///
    /// # Approach
    /// Iterate over digits of `n` via repeated `% 10` / `/ 10`. Maintain `first`
    /// (largest digit seen) and `second` (second largest). After each digit, update
    /// both values so that `first >= second` always holds.
    ///
    /// # Complexity
    /// - Time: O(log n) — number of digits in n
    /// - Space: O(1)
    pub fn max_product(n: i32) -> i32 {
        let (mut first, mut second) = (0, 0);
        let mut num = n;
        while num > 0 {
            let digit = num % 10;
            if digit >= first {
                second = first;
                first = digit;
            } else if digit > second {
                second = digit;
            }
            num /= 10;
        }
        first * second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_digits() {
        assert_eq!(Solution::max_product(31), 3);
    }

    #[test]
    fn test_repeated_digit() {
        assert_eq!(Solution::max_product(22), 4);
    }

    #[test]
    fn test_three_digits() {
        assert_eq!(Solution::max_product(124), 8);
    }

    #[test]
    fn test_with_zero_digit() {
        // digits: 1, 0 → max product = 1 * 0 = 0
        assert_eq!(Solution::max_product(10), 0);
    }

    #[test]
    fn test_all_same() {
        // digits: 9, 9, 9 → max product = 81
        assert_eq!(Solution::max_product(999), 81);
    }

    #[test]
    fn test_large_n() {
        // n = 1_000_000_009 → digits include 1, 0, ..., 9 → max = 9 * 1 = 9
        assert_eq!(Solution::max_product(1_000_000_009), 9);
    }

    #[test]
    fn test_max_constraint() {
        // n = 987_654_321 → largest digits are 9, 8 → product = 72
        assert_eq!(Solution::max_product(987_654_321), 72);
    }
}
