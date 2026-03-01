impl Solution {
    /// Sum all k-digit numbers whose digits are each drawn from [l, r].
    ///
    /// # Intuition
    /// Each of the k digit positions is filled independently from [l, r].
    /// The contribution of one position at power-of-10 weight `10^i` is
    /// `digit_sum * choices^(k-1)` (the other k−1 slots vary freely).
    /// Summing over all positions gives
    /// `digit_sum * choices^(k-1) * (10^k − 1) / 9`.
    ///
    /// # Approach
    /// 1. Compute `digit_sum = (l + r) * (r − l + 1) / 2` — the sum of all
    ///    digits in [l, r].
    /// 2. Compute `choices = r − l + 1`.
    /// 3. The geometric sum of positional weights is `(10^k − 1) / 9`;
    ///    division by 9 uses modular inverse via Fermat's little theorem.
    /// 4. Multiply: `digit_sum * choices^(k−1) * geo_sum (mod 10^9+7)`.
    ///
    /// # Complexity
    /// - Time: O(log k) for modular exponentiation.
    /// - Space: O(1).
    pub fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
            let mut result = 1i64;
            base %= modulus;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * base % modulus;
                }
                exp >>= 1;
                base = base * base % modulus;
            }
            result
        }

        let lorunavemi = (l as i64, r as i64, k as i64);
        let (l, r, k) = lorunavemi;

        let choices = r - l + 1;
        let digit_sum = (l + r) % MOD * (choices % MOD) % MOD * pow_mod(2, MOD - 2, MOD) % MOD;

        let geo_sum = (pow_mod(10, k, MOD) - 1 + MOD) % MOD * pow_mod(9, MOD - 2, MOD) % MOD;

        let choices_pow = pow_mod(choices, k - 1, MOD);

        (digit_sum % MOD * choices_pow % MOD * geo_sum % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::sum_of_numbers(1, 2, 2), 66);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::sum_of_numbers(0, 1, 3), 444);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::sum_of_numbers(5, 5, 10), 555555520);
    }

    #[test]
    fn single_digit_single_choice() {
        assert_eq!(Solution::sum_of_numbers(3, 3, 1), 3);
    }

    #[test]
    fn full_range_k1() {
        // digits 0..=9, k=1 → sum = 0+1+...+9 = 45
        assert_eq!(Solution::sum_of_numbers(0, 9, 1), 45);
    }

    #[test]
    fn large_k() {
        // k = 10^9, single digit 1 → number is 111...1 (10^9 ones)
        // = (10^(10^9) - 1) / 9 mod 10^9+7
        let result = Solution::sum_of_numbers(1, 1, 1_000_000_000);
        assert!(result >= 0 && result < 1_000_000_007);
    }
}
