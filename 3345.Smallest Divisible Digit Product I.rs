impl Solution {
    /// Linear scan for smallest number >= n whose digit product is divisible by t.
    ///
    /// # Intuition
    /// Given the tight constraint (n <= 100), a simple linear scan from n upward is
    /// well within bounds. For each candidate, compute the product of its digits and
    /// check divisibility by t. Note that any digit of 0 makes the product 0, which
    /// is divisible by every t.
    ///
    /// # Approach
    /// Iterate from n upward. For each number, extract its digits, multiply them
    /// together, and return the first number whose digit product is divisible by t.
    ///
    /// # Complexity
    /// - Time: O(1) — at most a handful of candidates given n <= 100
    /// - Space: O(1)
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        (n..)
            .find(|&num| {
                let product = Self::digit_product(num);
                product % t == 0
            })
            .unwrap_or(n)
    }

    fn digit_product(mut num: i32) -> i32 {
        let mut product = 1;
        while num > 0 {
            product *= num % 10;
            num /= 10;
        }
        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_zero_divisible() {
        // 10 has digit product 0, divisible by any t
        assert_eq!(Solution::smallest_number(10, 2), 10);
    }

    #[test]
    fn test_next_number_needed() {
        // 15 has digit product 5, not divisible by 3; 16 has product 6
        assert_eq!(Solution::smallest_number(15, 3), 16);
    }

    #[test]
    fn test_n_itself_qualifies() {
        // 1 has digit product 1, divisible by 1
        assert_eq!(Solution::smallest_number(1, 1), 1);
    }

    #[test]
    fn test_single_digit() {
        // 4 has digit product 4, divisible by 4
        assert_eq!(Solution::smallest_number(4, 4), 4);
    }

    #[test]
    fn test_must_advance() {
        // 11 has product 1, not divisible by 5; need to scan forward
        assert_eq!(Solution::smallest_number(11, 5), 15);
    }

    #[test]
    fn test_max_constraints() {
        // n = 100, t = 10: digit product of 100 is 0, divisible by 10
        assert_eq!(Solution::smallest_number(100, 10), 100);
    }
}
