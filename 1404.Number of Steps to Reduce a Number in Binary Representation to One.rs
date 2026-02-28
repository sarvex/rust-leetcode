impl Solution {
    /// Count operations with right-to-left carry simulation.
    ///
    /// # Intuition
    /// Directly applying `+1` and `/2` to a 500-bit number is inconvenient without big-integer
    /// arithmetic. Instead, we scan bits from right to left and keep a `carry` that represents
    /// whether a previous `+1` is still affecting higher bits.
    ///
    /// # Approach
    /// 1. Traverse bits from least significant to most significant, skipping the top bit.
    /// 2. For each bit, evaluate `effective = bit + carry`:
    ///    - `effective == 1`: odd -> perform `+1` and `/2` (`2` steps), set `carry = 1`
    ///    - `effective == 0` or `2`: even -> perform `/2` (`1` step), keep `carry` as is
    /// 3. Add the final `carry`:
    ///    - `0`: already reduced to `1`
    ///    - `1`: highest part is `10`, requiring one final `/2`
    ///
    /// # Complexity
    /// - Time: O(n), where `n = s.len()`
    /// - Space: O(1)
    pub fn num_steps(s: String) -> i32 {
        let (steps, carry) = s
            .as_bytes()
            .iter()
            .skip(1)
            .rev()
            .fold((0_i32, 0_i32), |(steps, carry), &bit| {
                if (bit - b'0') as i32 + carry == 1 {
                    (steps + 2, 1)
                } else {
                    (steps + 1, carry)
                }
            });

        steps + carry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::num_steps("10".to_string()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::num_steps("1".to_string()), 0);
    }

    #[test]
    fn all_ones_large() {
        assert_eq!(Solution::num_steps("1".repeat(500)), 501);
    }

    #[test]
    fn power_of_two_large() {
        let s = format!("1{}", "0".repeat(499));
        assert_eq!(Solution::num_steps(s), 499);
    }

    #[test]
    fn leetcode_long_case() {
        assert_eq!(
            Solution::num_steps("1111011110000011100000110001011011110010111001010111110001".to_string()),
            85
        );
    }
}
