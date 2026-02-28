impl Solution {
    /// Build the concatenated value using modular left shifts.
    ///
    /// # Intuition
    /// Appending the binary representation of `x` to an existing binary value `ans` is the same
    /// as shifting `ans` left by `bit_len(x)` and then adding `x`.
    ///
    /// # Approach
    /// 1. Iterate through values from `1` to `n`.
    /// 2. Increase the current bit length exactly when the value is a power of two.
    /// 3. Update the running answer with:
    ///    `ans = ((ans << bit_len) + value) % MOD`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        (1_i64..=n as i64)
            .fold((0_i64, 0_u32), |(ans, bit_len), value| {
                let next_bit_len = if value & (value - 1) == 0 {
                    bit_len + 1
                } else {
                    bit_len
                };

                let next_ans = ((ans << next_bit_len) + value) % MOD;
                (next_ans, next_bit_len)
            })
            .0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::concatenated_binary(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::concatenated_binary(3), 27);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::concatenated_binary(12), 505_379_714);
    }

    #[test]
    fn upper_bound_case() {
        assert_eq!(Solution::concatenated_binary(100_000), 757_631_812);
    }
}
