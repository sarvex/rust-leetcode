impl Solution {
    /// Smallest all-ones multiple: find minimal m so that (10^m - 1)/9 ≡ 0 (mod k), i.e. 10^m ≡ 1 (mod 9k).
    /// Return digit count m, or -1 when no solution (k divisible by 2 or 5).
    ///
    /// # Intuition
    /// Numbers 1, 11, 111, ... equal (10^m - 1)/9. For k to divide such an n we need 9k | (10^m - 1),
    /// i.e. 10^m ≡ 1 (mod 9k). The order of 10 mod 9k must divide φ(9k) by Euler's theorem.
    ///
    /// # Approach
    /// 1. If k % 2 == 0 or k % 5 == 0, return -1 (gcd(10, 9k) ≠ 1).
    /// 2. Compute φ(9k) via prime factorization.
    /// 3. Enumerate all divisors of φ(9k), sort ascending.
    /// 4. Return smallest divisor d where 10^d ≡ 1 (mod 9k) using fast exponentiation.
    ///
    /// # Complexity
    /// - Time: O(√k + d(φ) · log(φ)) where d(φ) is divisor count of φ(9k).
    /// - Space: O(d(φ)) for divisors.
    pub fn min_all_one_multiple(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }
        let m = 9 * (k as i64);
        let phi = Self::euler_totient(m);
        let mut divisors = Self::get_divisors(phi);
        divisors.sort_unstable();

        for d in divisors {
            if Self::mod_pow(10, d, m) == 1 {
                return d as i32;
            }
        }
        -1
    }

    fn euler_totient(mut n: i64) -> i64 {
        let mut result = n;
        let mut p = 2i64;
        while p * p <= n {
            if n % p == 0 {
                while n % p == 0 {
                    n /= p;
                }
                result -= result / p;
            }
            p += 1;
        }
        if n > 1 {
            result -= result / n;
        }
        result
    }

    fn get_divisors(n: i64) -> Vec<i64> {
        let mut divisors = Vec::new();
        let mut i = 1i64;
        while i * i <= n {
            if n % i == 0 {
                divisors.push(i);
                if i != n / i {
                    divisors.push(n / i);
                }
            }
            i += 1;
        }
        divisors
    }

    fn mod_pow(mut base: i64, mut exp: i64, modulo: i64) -> i64 {
        let mut result = 1i64;
        base %= modulo;
        while exp > 0 {
            if exp & 1 == 1 {
                result = ((result as i128) * (base as i128) % (modulo as i128)) as i64;
            }
            exp >>= 1;
            base = ((base as i128) * (base as i128) % (modulo as i128)) as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_all_one_multiple(3), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_all_one_multiple(7), 6);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_all_one_multiple(2), -1);
    }

    #[test]
    fn test_k_5() {
        assert_eq!(Solution::min_all_one_multiple(5), -1);
    }

    #[test]
    fn test_k_9() {
        assert_eq!(Solution::min_all_one_multiple(9), 9);
    }

    #[test]
    fn test_k_1() {
        assert_eq!(Solution::min_all_one_multiple(1), 1);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(Solution::min_all_one_multiple(99991), 49995);
    }
}
