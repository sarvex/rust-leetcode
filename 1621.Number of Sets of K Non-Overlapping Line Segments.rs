const MOD: i64 = 1_000_000_007;

impl Solution {
    /// Closed-form count via the binomial coefficient `C(n + k - 1, 2k)`.
    ///
    /// # Intuition
    /// Describe a valid drawing by its sorted endpoints. Segment `i` spans
    /// `[a_i, b_i]` with `a_i < b_i` (each segment covers at least two points),
    /// and consecutive segments may touch but not overlap, so `b_i <= a_{i+1}`.
    /// A whole configuration is therefore exactly one chain
    ///
    /// `0 <= a_1 < b_1 <= a_2 < b_2 <= ... <= a_k < b_k <= n - 1`
    ///
    /// which alternates strict and weak inequalities. Shifting the `i`-th
    /// segment's two endpoints right by `i - 1` turns every weak step into a
    /// strict one: setting `c_{2i-1} = a_i + (i - 1)` and `c_{2i} = b_i + (i - 1)`
    /// yields a strictly increasing sequence, because `b_i <= a_{i+1}` becomes
    /// `b_i + (i - 1) < a_{i+1} + i`. The largest shifted value is at most
    /// `(n - 1) + (k - 1) = n + k - 2`, so the `2k` values are distinct picks from
    /// `n + k - 1` slots. The map is reversible, so the count is `C(n + k - 1, 2k)`.
    ///
    /// # Approach
    /// 1. Set `total = n + k - 1` and `choose = 2k`; the constraint `k <= n - 1`
    ///    guarantees `choose <= total`.
    /// 2. Build factorials `0!..total!` modulo `MOD` with a running scan.
    /// 3. Invert the denominator `choose! * (total - choose)!` once using Fermat's
    ///    little theorem, then multiply by `total!`.
    ///
    /// # Complexity
    /// - Time: O(n + k) for the factorial table, plus O(log MOD) for the inverse
    /// - Space: O(n + k) for the factorial table
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let total = (n + k - 1) as usize;
        let choose = (2 * k) as usize;

        Self::n_choose_k(total, choose) as i32
    }

    /// Modular exponentiation by squaring.
    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1_i64;
        base %= MOD;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            exp >>= 1;
            base = base * base % MOD;
        }

        result
    }

    /// Binomial coefficient `C(n, k)` modulo `MOD`.
    ///
    /// Uses a factorial table and a single modular inverse of the combined
    /// denominator, since only one coefficient is needed.
    fn n_choose_k(n: usize, k: usize) -> i64 {
        if k > n {
            return 0;
        }

        let fact: Vec<i64> = std::iter::once(1_i64)
            .chain((1..=n as i64).scan(1_i64, |running, value| {
                *running = *running * value % MOD;
                Some(*running)
            }))
            .collect();

        let denominator = fact[k] * fact[n - k] % MOD;

        fact[n] * Self::mod_pow(denominator, MOD - 2) % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::number_of_sets(4, 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::number_of_sets(3, 1), 3);
    }

    #[test]
    fn example_three() {
        // C(36, 14) = 3_796_297_200, which wraps to 796_297_179 modulo 1e9+7.
        assert_eq!(Solution::number_of_sets(30, 7), 796_297_179);
    }

    #[test]
    fn minimum_input() {
        // Two points admit exactly one segment.
        assert_eq!(Solution::number_of_sets(2, 1), 1);
    }

    #[test]
    fn single_segment_counts_pairs() {
        // One segment over n points is any pair of endpoints: C(n, 2).
        assert_eq!(Solution::number_of_sets(1000, 1), 499_500);
    }

    #[test]
    fn maximum_segments() {
        // k = n - 1 forces every unit gap to be a segment: exactly one way.
        assert_eq!(Solution::number_of_sets(1000, 999), 1);
    }

    #[test]
    fn upper_bound_case() {
        assert_eq!(Solution::number_of_sets(1000, 500), 70_047_606);
    }
}
