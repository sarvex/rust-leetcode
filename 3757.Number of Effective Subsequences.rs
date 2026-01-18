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
        let mut idx = 0;
        for b in 0..20 {
            if (total_or >> b) & 1 == 1 {
                compress[b] = idx;
                idx += 1;
            }
        }

        // Count elements by compressed pattern
        let mut cnt = vec![0usize; 1 << k];
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

        // SOS DP
        for i in 0..k {
            for m in 0..(1 << k) {
                if (m >> i) & 1 == 1 {
                    cnt[m] += cnt[m ^ (1 << i)];
                }
            }
        }

        // Powers of 2
        let mut pow2 = vec![1u64; n + 1];
        for i in 1..=n {
            pow2[i] = pow2[i - 1] * 2 % MOD;
        }

        // Inclusion-exclusion for f[full]
        let mut non_eff = 0u64;
        for s in 0..=full {
            let pc = (full ^ s).count_ones();
            if pc & 1 == 0 {
                non_eff = (non_eff + pow2[cnt[s]]) % MOD;
            } else {
                non_eff = (non_eff + MOD - pow2[cnt[s]]) % MOD;
            }
        }

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
