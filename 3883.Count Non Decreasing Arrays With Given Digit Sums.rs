impl Solution {
    /// Counts non-decreasing arrays in [0, 5000] whose element-wise digit sums match the input.
    ///
    /// # Intuition
    /// A dense DP over the full value range wastes work on zero entries.  By keeping
    /// only populated `(value, ways)` pairs and merge-scanning them against the
    /// sorted bucket for the next required digit sum, every transition touches only
    /// the values that actually contribute.
    ///
    /// # Approach
    /// 1. Precompute buckets: for each digit sum *s*, collect all integers in
    ///    [0, 5000] whose digits total *s*, stored in ascending order.
    /// 2. Represent the DP state as a sparse sorted list of `(value, prefix_ways)`.
    ///    Initialise with a sentinel `(0, 1)` meaning "one way to start before any
    ///    choices, at value 0".
    /// 3. For each position, two-pointer merge the current `dp` with the target
    ///    bucket.  Accumulate a running prefix from `dp`; each bucket entry `v`
    ///    receives the prefix of all `dp` values ≤ `v`.  Retain only nonzero entries.
    /// 4. The answer is the sum of all counts in the final `dp`.
    ///
    /// # Complexity
    /// - Time:  O(R) precompute + O(n × B_max) runtime where R = 5001 and B_max ≤ 365
    /// - Space: O(R)
    pub fn count_arrays(digit_sum: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const MAX_VAL: usize = 5001;

        let mut buckets: Vec<Vec<u16>> = vec![Vec::new(); 51];
        for v in 0..MAX_VAL {
            let mut s = 0u32;
            let mut x = v;
            while x > 0 {
                s += (x % 10) as u32;
                x /= 10;
            }
            if (s as usize) < buckets.len() {
                buckets[s as usize].push(v as u16);
            }
        }

        for &ds in &digit_sum {
            if ds < 0 || ds as usize >= buckets.len() || buckets[ds as usize].is_empty() {
                return 0;
            }
        }

        // Sparse DP: (value, accumulated_ways)
        let mut dp: Vec<(u16, i64)> = vec![(0, 1)];

        for &ds in &digit_sum {
            let bucket = &buckets[ds as usize];
            let mut new_dp = Vec::with_capacity(bucket.len());
            let mut prefix: i64 = 0;
            let mut j = 0;

            for &v in bucket {
                // Absorb all dp entries with value <= v into prefix
                while j < dp.len() && dp[j].0 <= v {
                    prefix = (prefix + dp[j].1) % MOD;
                    j += 1;
                }
                if prefix != 0 {
                    new_dp.push((v, prefix));
                }
            }

            dp = new_dp;
        }

        (dp.iter().fold(0i64, |acc, &(_, c)| (acc + c) % MOD)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_arrays(vec![25, 1]), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_arrays(vec![1]), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_arrays(vec![2, 49, 23]), 0);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(Solution::count_arrays(vec![0]), 1);
    }

    #[test]
    fn test_non_decreasing_pairs() {
        assert_eq!(Solution::count_arrays(vec![1, 1]), 10);
    }

    #[test]
    fn test_impossible_digit_sum() {
        assert_eq!(Solution::count_arrays(vec![50]), 0);
    }

    #[test]
    fn test_decreasing_digit_sums() {
        assert_eq!(Solution::count_arrays(vec![1, 0]), 0);
    }

    #[test]
    fn test_double_zero() {
        assert_eq!(Solution::count_arrays(vec![0, 0]), 1);
    }

    #[test]
    fn test_three_elements() {
        assert_eq!(Solution::count_arrays(vec![1, 1, 1]), 20);
    }
}
