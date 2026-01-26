impl Solution {
    /// Combinatorics with modular arithmetic
    ///
    /// # Intuition
    /// Instead of iterating over all arrangements, use linearity: for each pair of cells,
    /// count arrangements where both have pieces and multiply by their Manhattan distance.
    ///
    /// # Approach
    /// 1. Each cell pair both has pieces in C(m*n-2, k-2) arrangements
    /// 2. Sum of Manhattan distances over all cell pairs = m * n * (m + n) * (m * n - 1) / 6
    /// 3. Use precomputed factorials and modular inverse for efficient computation
    ///
    /// # Complexity
    /// - Time: O(m * n) for factorial precomputation
    /// - Space: O(m * n) for factorial arrays
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
