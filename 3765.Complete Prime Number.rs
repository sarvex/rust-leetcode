impl Solution {
    /// Check whether every decimal prefix and suffix of `num` is prime.
    ///
    /// # Intuition
    /// A complete prime must satisfy primality for:
    /// - prefixes: `num`, `num / 10`, `num / 100`, ...
    /// - suffixes: `num % 10`, `num % 100`, ...
    /// We can generate both families with simple arithmetic and short-circuit on first failure.
    ///
    /// # Approach
    /// 1. Use trial division (`6k ± 1`) for primality checks.
    /// 2. Validate all prefixes by repeatedly dividing by 10.
    /// 3. Validate all suffixes by taking modulo powers of 10.
    /// 4. Return `true` only if every generated value is prime.
    ///
    /// # Complexity
    /// - Time: O(d * sqrt(num)) where `d` is number of digits (at most 10)
    /// - Space: O(1)
    pub fn complete_prime(num: i32) -> bool {
        if !Self::is_prime(i64::from(num)) {
            return false;
        }

        let mut prefix = num / 10;
        while prefix > 0 {
            if !Self::is_prime(i64::from(prefix)) {
                return false;
            }
            prefix /= 10;
        }

        let num_i64 = i64::from(num);
        let mut modulo_base = 10_i64;
        while modulo_base <= num_i64 {
            if !Self::is_prime(num_i64 % modulo_base) {
                return false;
            }
            modulo_base *= 10;
        }

        true
    }

    #[inline]
    fn is_prime(value: i64) -> bool {
        if value < 2 {
            return false;
        }
        if value == 2 || value == 3 {
            return true;
        }
        if value % 2 == 0 || value % 3 == 0 {
            return false;
        }

        let mut divisor = 5_i64;
        while divisor * divisor <= value {
            if value % divisor == 0 || value % (divisor + 2) == 0 {
                return false;
            }
            divisor += 6;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::complete_prime(23));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::complete_prime(39));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::complete_prime(7));
    }

    #[test]
    fn test_non_prime_single_digit() {
        assert!(!Solution::complete_prime(1));
    }

    #[test]
    fn test_prefix_fails() {
        assert!(!Solution::complete_prime(101));
    }

    #[test]
    fn test_suffix_fails() {
        assert!(!Solution::complete_prime(233));
    }

    #[test]
    fn test_multiple_digits_all_valid() {
        assert!(Solution::complete_prime(313));
    }
}
