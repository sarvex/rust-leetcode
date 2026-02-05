impl Solution {
    /// Maximizes the product of integers that sum to n using mathematical insight.
    ///
    /// # Intuition
    /// The optimal strategy is to break n into as many 3s as possible:
    /// - 3 * 3 > 2 * 2 * 2 (9 > 8), so prefer 3s over 2s
    /// - Never use 1 (1 * k < k for any k > 1)
    /// - Handle remainders: if n % 3 == 1, use one 4 instead of 1 * 3
    ///
    /// # Approach
    /// 1. Special cases: n=2 returns 1, n=3 returns 2
    /// 2. If n % 3 == 0: result = 3^(n/3)
    /// 3. If n % 3 == 1: result = 3^((n/3)-1) * 4
    /// 4. If n % 3 == 2: result = 3^(n/3) * 2
    ///
    /// # Complexity
    /// - Time: O(log n) for exponentiation
    /// - Space: O(1)
    pub fn integer_break(n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            _ => {
                let quotient = n / 3;
                match n % 3 {
                    0 => 3_i32.pow(quotient as u32),
                    1 => 3_i32.pow((quotient - 1) as u32) * 4,
                    _ => 3_i32.pow(quotient as u32) * 2,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_two() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn break_three() {
        assert_eq!(Solution::integer_break(3), 2);
    }

    #[test]
    fn break_four() {
        // 4 = 2 + 2, product = 4
        assert_eq!(Solution::integer_break(4), 4);
    }

    #[test]
    fn break_ten() {
        // 10 = 3 + 3 + 4, product = 36
        assert_eq!(Solution::integer_break(10), 36);
    }

    #[test]
    fn break_six() {
        // 6 = 3 + 3, product = 9
        assert_eq!(Solution::integer_break(6), 9);
    }

    #[test]
    fn break_seven() {
        // 7 = 3 + 4, product = 12 or 7 = 3 + 2 + 2 = 12
        assert_eq!(Solution::integer_break(7), 12);
    }

    #[test]
    fn break_eight() {
        // 8 = 3 + 3 + 2, product = 18
        assert_eq!(Solution::integer_break(8), 18);
    }

    #[test]
    fn break_large() {
        // 58 = 19 * 3 + 1 = 3^18 * 4
        assert_eq!(Solution::integer_break(58), 1549681956);
    }
}
