impl Solution {
    /// Computes XOR of array after range multiplication queries with sqrt decomposition.
    ///
    /// # Intuition
    /// Partition queries at threshold sqrt(n). For small k, group by (k, residue) and
    /// only propagate multiplicative diffs along residue classes that have queries.
    /// For large k, apply directly since each touches at most sqrt(n) elements.
    ///
    /// # Approach
    /// 1. Group small-k queries by (k, l%k) into residue classes
    /// 2. For each active residue class: build diff array, propagate prefix products
    ///    along that stride, apply to nums, then sparse-reset only touched positions
    /// 3. Large-k queries: direct step-by multiplication
    ///
    /// # Complexity
    /// - Time: O((n + q) * sqrt(n))
    /// - Space: O(n) for the difference buffer
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut nums: Vec<i64> = nums.into_iter().map(i64::from).collect();
        let n = nums.len();
        let t = (n as f64).sqrt() as usize + 1;

        // (k, residue) -> list of (l, r_next, v) for diff array operations
        let mut groups: Vec<Vec<(usize, usize, i64)>> = Vec::new();
        let mut key_to_idx: Vec<i64> = Vec::new();
        let mut key_map = std::collections::HashMap::<i64, usize>::new();

        for q in &queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as i64;
            if k < t {
                let residue = l % k;
                let key = ((k as i64) << 20) | residue as i64;
                let idx = match key_map.get(&key) {
                    Some(&i) => i,
                    None => {
                        let i = groups.len();
                        key_map.insert(key, i);
                        key_to_idx.push(key);
                        groups.push(Vec::new());
                        i
                    }
                };
                let r_next = ((r - l) / k + 1) * k + l;
                groups[idx].push((l, r_next, v));
            } else {
                let mut i = l;
                while i <= r {
                    nums[i] = nums[i] * v % MOD;
                    i += k;
                }
            }
        }

        let mut dif = vec![1i64; n + t];
        for (gi, group) in groups.iter().enumerate() {
            let key = key_to_idx[gi];
            let k = (key >> 20) as usize;
            let residue = (key & 0xF_FFFF) as usize;

            for &(l, r_next, v) in group {
                dif[l] = dif[l] * v % MOD;
                dif[r_next] = dif[r_next] * Self::pow_mod(v, MOD - 2, MOD) % MOD;
            }

            // Propagate and apply only along this residue class stride
            let mut j = residue + k;
            while j < n {
                dif[j] = dif[j] * dif[j - k] % MOD;
                j += k;
            }
            j = residue;
            while j < n {
                nums[j] = nums[j] * dif[j] % MOD;
                j += k;
            }

            // Sparse reset: only indices along this residue's stride + overflow
            j = residue;
            while j < n + t {
                dif[j] = 1;
                j += k;
            }
        }

        nums.into_iter().fold(0, |acc, x| acc ^ x as i32)
    }

    #[inline]
    fn pow_mod(mut x: i64, mut y: i64, m: i64) -> i64 {
        let mut res = 1i64;
        while y > 0 {
            if y & 1 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            y >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_query_multiplies_all_elements() {
        let nums = vec![1, 1, 1];
        let queries = vec![vec![0, 2, 1, 4]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 4);
    }

    #[test]
    fn multiple_queries_with_step_gaps() {
        let nums = vec![2, 3, 1, 5, 4];
        let queries = vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 31);
    }

    #[test]
    fn step_exceeds_range_single_application() {
        let nums = vec![5, 10, 15];
        let queries = vec![vec![0, 2, 5, 2]];
        assert_eq!(Solution::xor_after_queries(nums, queries), 10 ^ 15 ^ 10);
    }

    #[test]
    fn large_multiplier_with_modulo_arithmetic() {
        let nums = vec![1_000_000_000];
        let queries = vec![vec![0, 0, 1, 100_000]];
        let expected = ((1_000_000_000i64 * 100_000) % 1_000_000_007) as i32;
        assert_eq!(Solution::xor_after_queries(nums, queries), expected);
    }

    #[test]
    fn empty_queries_returns_original_xor() {
        let nums = vec![1, 2, 3, 4, 5];
        let queries: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::xor_after_queries(nums, queries),
            1 ^ 2 ^ 3 ^ 4 ^ 5
        );
    }
}
