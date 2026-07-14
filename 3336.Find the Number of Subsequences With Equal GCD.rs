impl Solution {
    /// In-place DP bounded by max(nums) with precomputed GCD table.
    ///
    /// # Intuition
    /// Each element joins seq1, joins seq2, or is skipped. Tracking (gcd1, gcd2)
    /// as DP state suffices because GCD values are always divisors of the input,
    /// so the reachable state space is bounded by max(nums), not a fixed 200.
    ///
    /// # Approach
    /// 1. Let m = max(nums). GCD values live in [0, m], so the table is (m+1)².
    /// 2. Precompute gcd_table[a][b] for all a, b in [0, m] once.
    /// 3. Use two flat buffers (dp, scratch) allocated once; swap via copy+fill
    ///    each round — zero heap allocation in the hot loop.
    /// 4. For each value v and every nonzero cell (j, k):
    ///    - skip:  dp[j][k]              += ways
    ///    - seq1:  dp[gcd(j,v)][k]       += ways
    ///    - seq2:  dp[j][gcd(k,v)]       += ways
    /// 5. Answer = Σ dp[g][g] for g in [1, m].
    ///
    /// # Complexity
    /// - Time: O(n · m²) where m = max(nums) ≤ 200
    /// - Space: O(m²)
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let m = *nums.iter().max().unwrap() as usize;
        let size = m + 1;

        // Precompute GCD table over [0, m] — pays for itself in the O(n·m²) loop.
        let gcd_table: Vec<Vec<u8>> = (0..size)
            .map(|a| (0..size).map(|b| Self::gcd(a, b) as u8).collect())
            .collect();

        // Two flat buffers; copy_from_slice + fill replaces per-element allocation.
        let mut dp = vec![0i32; size * size];
        let mut scratch = vec![0i32; size * size];
        dp[0] = 1; // dp[0][0] = 1

        for &num in &nums {
            let v = num as usize;

            scratch.copy_from_slice(&dp);
            dp.fill(0);

            for j in 0..size {
                let dj = gcd_table[j][v] as usize;
                for k in 0..size {
                    let val = scratch[j * size + k];
                    if val == 0 {
                        continue;
                    }
                    let dk = gcd_table[k][v] as usize;

                    // Skip
                    dp[j * size + k] = (dp[j * size + k] + val) % MOD;
                    // Assign to seq1
                    dp[dj * size + k] = (dp[dj * size + k] + val) % MOD;
                    // Assign to seq2
                    dp[j * size + dk] = (dp[j * size + dk] + val) % MOD;
                }
            }
        }

        (1..size).fold(0, |acc, g| (acc + dp[g * size + g]) % MOD)
    }

    #[inline(always)]
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::subsequence_pair_count(vec![5]), 0);
    }

    #[test]
    fn test_two_equal_elements() {
        assert_eq!(Solution::subsequence_pair_count(vec![3, 3]), 1);
    }

    #[test]
    fn test_coprime_elements() {
        assert_eq!(Solution::subsequence_pair_count(vec![2, 3]), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::subsequence_pair_count(vec![7, 7, 7]), 6);
    }
}
