impl Solution {
    /// Möbius Transform over Subset Lattice
    ///
    /// # Intuition
    /// A subsequence is effective iff removing it strictly decreases the OR of remaining
    /// elements. We count non-effective subsequences (where remaining OR = total_or) and
    /// subtract from 2^n.
    ///
    /// # Approach
    /// Use subset sum DP and Möbius inversion. Compress to only bits present in total_or.
    /// g[mask] = 2^(elements with pattern ⊆ mask), f[full] via inclusion-exclusion.
    ///
    /// # Complexity
    /// - Time: O(n + 2^k * k) where k = popcount(total_or) ≤ 20
    /// - Space: O(n + 2^k)

    pub fn count_effective(nums: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = nums.len();

        let total_or: i32 = nums.iter().fold(0, |a, &x| a | x);
        if total_or == 0 {
            return 0;
        }

        let k = total_or.count_ones() as usize;
        let full = (1usize << k) - 1;

        // Build compression map: bit position -> compressed index
        let mut compress = [0usize; 20];
        (0..20)
            .filter(|&b| (total_or >> b) & 1 == 1)
            .enumerate()
            .for_each(|(idx, b)| compress[b] = idx);

        // Count elements by compressed pattern (u32 for cache efficiency; max count ≤ n)
        let size = 1 << k;
        let mut cnt = vec![0u32; size];
        for &num in &nums {
            let mut c = 0usize;
            let mut v = num & total_or;
            while v != 0 {
                let b = v.trailing_zeros() as usize;
                c |= 1 << compress[b];
                v &= v - 1;
            }
            cnt[c] += 1;
        }

        // SOS DP: in-place, cache-friendly order
        for i in 0..k {
            let bit = 1 << i;
            for m in 0..size {
                if (m & bit) != 0 {
                    cnt[m] += cnt[m ^ bit];
                }
            }
        }

        // Powers of 2
        let pow2: Vec<u64> = std::iter::successors(Some(1u64), |&x| Some((x * 2) % MOD))
            .take(n + 1)
            .collect();

        // Inclusion-exclusion for f[full]: branchless with batched modulo
        const BATCH: usize = 512;
        let k_u32 = k as u32;
        let mut even_sum = 0u64;
        let mut odd_sum = 0u64;
        for s in 0..=full {
            let val = pow2[cnt[s] as usize];
            let parity = (k_u32 - s.count_ones()) & 1;
            even_sum += val * (1 - parity as u64);
            odd_sum += val * parity as u64;
            if (s + 1) % BATCH == 0 {
                even_sum %= MOD;
                odd_sum %= MOD;
            }
        }
        let non_eff = (even_sum % MOD + MOD - odd_sum % MOD) % MOD;

        ((pow2[n] + MOD - non_eff) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_effective(vec![1, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_effective(vec![7, 4, 6]), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::count_effective(vec![8, 8]), 1);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::count_effective(vec![2, 2, 1]), 5);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::count_effective(vec![5]), 1);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::count_effective(vec![3, 3, 3]), 1);
    }
}
