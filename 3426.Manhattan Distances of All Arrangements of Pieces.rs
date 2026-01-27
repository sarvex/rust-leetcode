impl Solution {
    /// Computes total Manhattan distance across all k-piece arrangements on an m×n grid.
    ///
    /// # Intuition
    /// By linearity of expectation, sum over all cell pairs the number of
    /// arrangements containing both cells times their Manhattan distance.
    /// Each pair appears in C(m·n−2, k−2) arrangements.
    ///
    /// # Approach
    /// 1. Precompute factorials and inverse factorials modulo 10^9+7.
    /// 2. The sum of Manhattan distances over all cell pairs equals
    ///    m·n·(m+n)·(m·n−1)/6.
    /// 3. Multiply by C(m·n−2, k−2).
    ///
    /// # Complexity
    /// - Time: O(m × n) for factorial precomputation
    /// - Space: O(m × n) for factorial arrays
    pub fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let m = m as i64;
        let n = n as i64;
        let k = k as i64;
        let total = m * n;
        let max_n = total as usize;

        let mut fact = vec![1i64; max_n + 1];
        (1..=max_n).for_each(|i| fact[i] = fact[i - 1] * i as i64 % MOD);

        let mod_pow = |mut base: i64, mut exp: i64| -> i64 {
            let mut result = 1;
            base %= MOD;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * base % MOD;
                }
                exp >>= 1;
                base = base * base % MOD;
            }
            result
        };

        let mut inv_fact = vec![1i64; max_n + 1];
        inv_fact[max_n] = mod_pow(fact[max_n], MOD - 2);
        (0..max_n)
            .rev()
            .for_each(|i| inv_fact[i] = inv_fact[i + 1] * (i + 1) as i64 % MOD);

        let comb = |n: usize, r: usize| -> i64 {
            if r > n {
                0
            } else {
                fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
            }
        };

        let c = comb((total - 2) as usize, (k - 2) as usize);
        let inv6 = mod_pow(6, MOD - 2);
        let sum_dist =
            m % MOD * (n % MOD) % MOD * ((m + n) % MOD) % MOD * ((total - 1) % MOD) % MOD * inv6
                % MOD;

        (sum_dist * c % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_grid_two_pieces() {
        assert!(Solution::distance_sum(2, 2, 2) > 0);
    }

    #[test]
    fn single_row_grid() {
        assert!(Solution::distance_sum(1, 5, 2) > 0);
    }

    #[test]
    fn single_column_grid() {
        assert!(Solution::distance_sum(5, 1, 3) > 0);
    }

    #[test]
    fn minimum_pieces() {
        assert_eq!(Solution::distance_sum(2, 2, 2), 8);
    }
}
