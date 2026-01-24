use std::collections::HashMap;

impl Solution {
    /// Computes XOR of array after applying range multiplication queries with step increments.
    ///
    /// # Intuition
    /// Direct simulation is O(n*q) which TLEs. Use sqrt decomposition: for small step
    /// sizes, batch updates using multiplicative difference arrays with modular inverse.
    /// For large step sizes, direct simulation is efficient (few elements per query).
    ///
    /// # Approach
    /// 1. Split queries by step size k using threshold √n
    /// 2. For k ≤ √n: Group by (k, residue), use difference arrays with mod inverse
    /// 3. For k > √n: Direct simulation (O(√n) elements per query)
    /// 4. Combine multipliers and compute final XOR
    ///
    /// # Complexity
    /// - Time: O((n + q) * √n) amortized
    /// - Space: O(n) for multiplier arrays
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();

        let bravexuneth = (&nums, &queries);
        let _ = bravexuneth;

        let mod_pow = |mut base: i64, mut exp: i64| -> i64 {
            let mut result = 1i64;
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

        let mod_inverse = |a: i64| mod_pow(a, MOD - 2);

        let threshold = ((n as f64).sqrt() as usize).max(1);

        let mut small_k_queries: HashMap<(usize, usize), Vec<(usize, usize, i64)>> = HashMap::new();
        let mut mult = vec![1i64; n];

        queries.iter().for_each(|query| {
            let (l, r, k, v) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as i64,
            );

            if k <= threshold {
                let residue = l % k;
                let start_pos = l / k;
                let last_idx = l + (r - l) / k * k;
                let end_pos = last_idx / k;
                small_k_queries
                    .entry((k, residue))
                    .or_default()
                    .push((start_pos, end_pos, v));
            } else {
                (l..=r)
                    .step_by(k)
                    .for_each(|idx| mult[idx] = mult[idx] * v % MOD);
            }
        });

        small_k_queries
            .into_iter()
            .for_each(|((k, residue), qs)| {
                let max_pos = (n + k - 1) / k + 1;
                let mut diff = vec![1i64; max_pos + 1];

                qs.into_iter().for_each(|(start_pos, end_pos, v)| {
                    diff[start_pos] = diff[start_pos] * v % MOD;
                    if end_pos + 1 < diff.len() {
                        diff[end_pos + 1] = diff[end_pos + 1] * mod_inverse(v) % MOD;
                    }
                });

                let mut prefix = 1i64;
                (0..max_pos).for_each(|pos| {
                    prefix = prefix * diff[pos] % MOD;
                    let idx = residue + pos * k;
                    if idx < n {
                        mult[idx] = mult[idx] * prefix % MOD;
                    }
                });
            });

        (0..n)
            .map(|i| (nums[i] as i64) * mult[i] % MOD)
            .fold(0i64, |acc, x| acc ^ x) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_query_all_elements() {
        let nums = vec![1, 1, 1];
        let queries = vec![vec![0, 2, 1, 4]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 4);
    }

    #[test]
    fn test_multiple_queries() {
        let nums = vec![2, 3, 1, 5, 4];
        let queries = vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 31);
    }

    #[test]
    fn test_step_greater_than_range() {
        let nums = vec![5, 10, 15];
        let queries = vec![vec![0, 2, 5, 2]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 10 ^ 15 ^ 10);
    }

    #[test]
    fn test_large_multiplier() {
        let nums = vec![1_000_000_000];
        let queries = vec![vec![0, 0, 1, 100_000]];
        let expected = ((1_000_000_000i64 * 100_000) % 1_000_000_007) as i32;
        assert_eq!(Solution::xor_after_queries(nums, queries), expected);
    }

    #[test]
    fn test_empty_queries() {
        let nums = vec![1, 2, 3, 4, 5];
        let queries: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::xor_after_queries(nums, queries),
            1 ^ 2 ^ 3 ^ 4 ^ 5
        );
    }
}
