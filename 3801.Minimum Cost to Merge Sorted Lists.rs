impl Solution {
    /// Bitmask DP with coordinate-compressed median computation
    ///
    /// # Intuition
    /// With at most 12 lists, bitmask DP explores all merge orders. Computing medians via
    /// binary search over coordinate-compressed values avoids expensive merge operations.
    ///
    /// # Approach
    /// 1. Coordinate compress all values and precompute prefix counts per list
    /// 2. For each subset, binary search to find median using cumulative counts
    /// 3. Use bitmask DP with subset enumeration, fixing LSB to avoid duplicate pairs
    ///
    /// # Complexity
    /// - Time: O(2^n * v * log(v)) for median precomputation + O(3^n) for DP
    /// - Space: O(n * v + 2^n) for counts and DP arrays
    pub fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {
        let n = lists.len();
        let nmask = 1usize << n;

        let mut values: Vec<i32> = lists.iter().flat_map(|a| a.iter().copied()).collect();
        values.sort_unstable();
        values.dedup();
        let v = values.len();

        let lens: Vec<u16> = lists.iter().map(|a| a.len() as u16).collect();

        let mut cnt = vec![0u16; n * v];
        for (i, a) in lists.iter().enumerate() {
            let mut p = 0usize;
            let base = i * v;
            for (t, &val) in values.iter().enumerate() {
                while p < a.len() && a[p] <= val {
                    p += 1;
                }
                cnt[base + t] = p as u16;
            }
        }

        let mut total_len = vec![0u16; nmask];
        let mut total_len_i64 = vec![0i64; nmask];
        let mut k_mask = vec![0usize; nmask];

        let mut members_base = vec![[0u16; 12]; nmask];
        let mut mcount = vec![0u8; nmask];

        for mask in 1..nmask {
            let lsb = mask & mask.wrapping_neg();
            let i = lsb.trailing_zeros() as usize;
            let pm = mask ^ lsb;

            let lenm = total_len[pm] + lens[i];
            total_len[mask] = lenm;
            total_len_i64[mask] = lenm as i64;
            k_mask[mask] = ((lenm as usize).wrapping_sub(1)) >> 1;

            let c = mcount[pm] as usize;
            members_base[mask] = members_base[pm];
            members_base[mask][c] = (i * v) as u16;
            mcount[mask] = (c + 1) as u8;
        }

        let median_for_mask = |bases: &[u16; 12], cnt_bases: usize, k: usize| -> usize {
            let (mut lo, mut hi) = (0usize, v - 1);
            while lo < hi {
                let mid = (lo + hi) >> 1;
                let mut s: usize = 0;
                for t in 0..cnt_bases {
                    s += cnt[bases[t] as usize + mid] as usize;
                }
                if s > k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            lo
        };

        let mut median: Vec<i32> = vec![0; nmask];
        for mask in 1..nmask {
            let idx = median_for_mask(&members_base[mask], mcount[mask] as usize, k_mask[mask]);
            median[mask] = values[idx];
        }

        let inf: i64 = i64::MAX / 4;
        let mut dp = vec![inf; nmask];
        (0..n).for_each(|i| dp[1usize << i] = 0);

        for mask in 1..nmask {
            if mask & (mask - 1) == 0 {
                continue;
            }
            let fixed = mask & mask.wrapping_neg();
            let lenm = total_len_i64[mask];

            let mut best = inf;
            let mut sub = (mask - 1) & mask;
            while sub != 0 {
                if (sub & fixed) != 0 {
                    let other = mask ^ sub;
                    let diff = (median[sub] as i64 - median[other] as i64).abs();
                    let cost = dp[sub] + dp[other] + lenm + diff;
                    best = best.min(cost);
                }
                sub = (sub - 1) & mask;
            }
            dp[mask] = best;
        }

        dp[nmask - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_lists_merge() {
        let lists = vec![vec![1, 3, 5], vec![2, 4], vec![6, 7, 8]];
        assert_eq!(Solution::min_merge_cost(lists), 18);
    }

    #[test]
    fn test_overlapping_values() {
        let lists = vec![vec![1, 1, 5], vec![1, 4, 7, 8]];
        assert_eq!(Solution::min_merge_cost(lists), 10);
    }

    #[test]
    fn test_two_singletons() {
        let lists = vec![vec![1], vec![3]];
        assert_eq!(Solution::min_merge_cost(lists), 4);
    }

    #[test]
    fn test_identical_singletons() {
        let lists = vec![vec![1], vec![1]];
        assert_eq!(Solution::min_merge_cost(lists), 2);
    }
}
