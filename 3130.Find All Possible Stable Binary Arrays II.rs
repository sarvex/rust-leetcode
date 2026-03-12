const MOD: u64 = 1_000_000_007;
const N: usize = 2001;

const fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

const fn precalc() -> ([u64; N], [u64; N]) {
    let mut fact = [1_u64; N];
    let mut inv_fact = [1_u64; N];
    let mut i = 2;
    while i < N {
        fact[i] = fact[i - 1] * i as u64 % MOD;
        i += 1;
    }
    inv_fact[N - 1] = mod_pow(fact[N - 1], MOD - 2);
    let mut i = N - 1;
    while i > 1 {
        inv_fact[i - 1] = inv_fact[i] * i as u64 % MOD;
        i -= 1;
    }
    (fact, inv_fact)
}

const TABLES: ([u64; N], [u64; N]) = precalc();

/// Binomial coefficient C(n, r) mod MOD via precomputed factorials.
fn c(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }
    TABLES.0[n as usize] * (TABLES.1[r as usize] * TABLES.1[(n - r) as usize] % MOD) % MOD
}

/// Counts compositions of `n` into `k` parts in \[1, limit\] via inclusion-exclusion.
fn split_ways(n: u64, k: u64, limit: u64) -> u64 {
    if n == k {
        return 1;
    }
    if n > k * limit {
        return 0;
    }
    let (mut total, mut sign, mut remaining, mut j) = (0_u64, true, n, 0_u64);
    while j <= k && k <= remaining {
        let t = c(k, j) * c(remaining - 1, k - 1) % MOD;
        total = (total + if sign { t } else { MOD - t }) % MOD;
        sign = !sign;
        if remaining <= limit {
            break;
        }
        remaining -= limit;
        j += 1;
    }
    total
}

impl Solution {
    /// Counts stable binary arrays using combinatorial inclusion-exclusion.
    ///
    /// # Intuition
    /// Instead of position-by-position DP, enumerate how many runs of 0s there are
    /// (call it k). The runs of 1s must be k−1, k, or k+1 depending on whether the
    /// array starts and/or ends with 0. Counting compositions into bounded parts
    /// reduces to inclusion-exclusion over binomial coefficients.
    ///
    /// # Approach
    /// 1. Precompute factorials and inverse factorials at compile time for O(1) binomial lookups.
    /// 2. `split_ways(n, k, limit)` counts compositions of n into k parts in \[1, limit\]
    ///    via inclusion-exclusion in O(min(k, n/limit)) time.
    /// 3. The main loop iterates k = 1..=min(zero, one), combining zero-side and one-side
    ///    split counts with coefficients 1, 2, 1 for the three interleaving patterns.
    ///
    /// # Complexity
    /// - Time:  O(min(zero, one) × max(zero, one) / limit)
    /// - Space: O(1) working (factorial tables are compile-time constants)
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (zero, one, limit) = (zero.min(one) as u64, zero.max(one) as u64, limit as u64);
        if limit == 1 {
            if zero == one {
                return 2;
            }
            if zero + 1 == one {
                return 1;
            }
            return 0;
        }
        let (mut result, mut prev, mut curr, mut next) = (
            0_u64,
            0_u64,
            split_ways(one, 1, limit),
            split_ways(one, 2, limit),
        );
        for k in 1..=zero {
            result = (result + (prev + 2 * curr + next) % MOD * split_ways(zero, k, limit)) % MOD;
            (prev, curr, next) = (curr, next, split_ways(one, k + 2, limit));
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }

    #[test]
    fn test_limit_one_alternating() {
        assert_eq!(Solution::number_of_stable_arrays(2, 2, 1), 2);
    }

    #[test]
    fn test_large_limit_no_constraint() {
        assert_eq!(Solution::number_of_stable_arrays(2, 3, 5), 10);
    }

    #[test]
    fn test_max_params() {
        assert_eq!(
            Solution::number_of_stable_arrays(1000, 1000, 1000),
            72_475_738
        );
    }

    #[test]
    fn test_max_params_limit_one() {
        assert_eq!(Solution::number_of_stable_arrays(1000, 1000, 1), 2);
    }
}
