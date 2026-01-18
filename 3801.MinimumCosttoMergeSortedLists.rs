impl Solution {
    /// Minimum Cost to Merge Sorted Lists using Bitmask DP
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

        // cnt[i * v + t] = count of elements <= values[t] in lists[i]
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
            k_mask[mask] = ((lenm as usize) - 1) >> 1;

            let c = mcount[pm] as usize;
            members_base[mask] = members_base[pm];
            members_base[mask][c] = (i * v) as u16;
            mcount[mask] = (c + 1) as u8;
        }

        #[inline(always)]
        unsafe fn median_for_mask(
            cnt_ptr: *const u16,
            bases: *const u16,
            cnt_bases: usize,
            v: usize,
            k: usize,
        ) -> usize {
            let mut lo = 0usize;
            let mut hi = v - 1;
            while lo < hi {
                let mid = (lo + hi) >> 1;
                let mut s = 0usize;
                for t in 0..cnt_bases {
                    let base = *bases.add(t) as usize;
                    s += *cnt_ptr.add(base + mid) as usize;
                }
                if s > k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            lo
        }

        let mut median = vec![0i32; nmask];
        unsafe {
            let cnt_ptr = cnt.as_ptr();
            let val_ptr = values.as_ptr();
            for mask in 1..nmask {
                let idx = median_for_mask(
                    cnt_ptr,
                    members_base[mask].as_ptr(),
                    mcount[mask] as usize,
                    v,
                    k_mask[mask],
                );
                median[mask] = *val_ptr.add(idx);
            }
        }

        let inf: i64 = i64::MAX / 4;
        let mut dp = vec![inf; nmask];
        for i in 0..n {
            dp[1usize << i] = 0;
        }

        unsafe {
            let dp_ptr = dp.as_mut_ptr();
            let med_ptr = median.as_ptr();
            let len_ptr = total_len_i64.as_ptr();

            for mask in 1..nmask {
                if mask & (mask - 1) == 0 {
                    continue;
                }
                let fixed = mask & mask.wrapping_neg();
                let lenm = *len_ptr.add(mask);

                let mut best = inf;
                let mut sub = (mask - 1) & mask;
                while sub != 0 {
                    if (sub & fixed) != 0 {
                        let other = mask ^ sub;

                        let a = *med_ptr.add(sub) as i64;
                        let b = *med_ptr.add(other) as i64;
                        let diff = if a >= b { a - b } else { b - a };

                        let cost = *dp_ptr.add(sub) + *dp_ptr.add(other) + lenm + diff;
                        if cost < best {
                            best = cost;
                        }
                    }
                    sub = (sub - 1) & mask;
                }
                *dp_ptr.add(mask) = best;
            }
        }

        dp[nmask - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let lists = vec![vec![1, 3, 5], vec![2, 4], vec![6, 7, 8]];
        assert_eq!(Solution::min_merge_cost(lists), 18);
    }

    #[test]
    fn test_example_2() {
        let lists = vec![vec![1, 1, 5], vec![1, 4, 7, 8]];
        assert_eq!(Solution::min_merge_cost(lists), 10);
    }

    #[test]
    fn test_example_3() {
        let lists = vec![vec![1], vec![3]];
        assert_eq!(Solution::min_merge_cost(lists), 4);
    }

    #[test]
    fn test_example_4() {
        let lists = vec![vec![1], vec![1]];
        assert_eq!(Solution::min_merge_cost(lists), 2);
    }
}
