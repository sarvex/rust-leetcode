impl Solution {
    /// Sparse table RMQ over zero-run groups for O((n + q) log n) query processing.
    ///
    /// # Intuition
    /// A trade sacrifices one inner 1-run (converts it to 0s), then converts one
    /// 0-run (now surrounded by 1s) to 1s. The net gain in the full string equals
    /// `(converted 0-run size within query) - (sacrificed 1-run size)`. The answer
    /// for each query is `total_ones_in_s + max_gain`.
    ///
    /// # Approach
    /// 1. Precompute global zero-runs and inner 1-runs (gaps between consecutive zero-runs).
    /// 2. Build three sparse tables:
    ///    - `max_adj`: max of `z[i] + z[i+1]` for adjacent zero-run pairs (merged-run gain).
    ///    - `max_zero`: max single zero-run length (best target after any sacrifice).
    ///    - `min_one`: min inner 1-run length (cheapest sacrifice).
    /// 3. Per query `[l, r]`, binary-search to find overlapping zero-runs `[fz, lz]`.
    ///    Clip boundary runs to the query window, then compute:
    ///    - **Option A** (adjacent pair): `clipped(z[k]) + clipped(z[k+1])` for best `k`.
    ///    - **Option B** (independent choice): `max_zero_in_query - min_inner_one_in_query`.
    ///      Return `total_ones + max(0, best_gain)`.
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n)
    /// - Space: O(n log n)
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let total_ones: i32 = s.iter().filter(|&&c| c == b'1').count() as i32;

        // Build zero-runs: (start, end, length), sorted by start position.
        let mut zero_runs: Vec<(u32, u32, i32)> = Vec::with_capacity(n / 2 + 1);
        let mut i = 0usize;
        while i < n {
            if s[i] == b'0' {
                let start = i;
                while i < n && s[i] == b'0' {
                    i += 1;
                }
                zero_runs.push((start as u32, (i - 1) as u32, (i - start) as i32));
            } else {
                i += 1;
            }
        }

        let nz = zero_runs.len();
        if nz == 0 {
            return queries.iter().map(|_| total_ones).collect();
        }

        let zero_lens: Vec<i32> = zero_runs.iter().map(|r| r.2).collect();

        // inner_ones[i] = 1-run length between zero_runs[i] and zero_runs[i+1].
        let inner_ones: Vec<i32> = (0..nz.saturating_sub(1))
            .map(|i| (zero_runs[i + 1].0 - zero_runs[i].1 - 1) as i32)
            .collect();

        // adj_sums[i] = z[i] + z[i+1]: gain when merging adjacent pair i and i+1.
        let adj_sums: Vec<i32> = (0..nz.saturating_sub(1))
            .map(|i| zero_lens[i] + zero_lens[i + 1])
            .collect();

        let max_adj_st = SparseTable::new(&adj_sums, true);
        let max_zero_st = SparseTable::new(&zero_lens, true);
        let min_one_st = SparseTable::new(&inner_ones, false);

        queries
            .iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                let gain = Self::compute_gain(
                    l,
                    r,
                    &zero_runs,
                    &zero_lens,
                    &max_adj_st,
                    &max_zero_st,
                    &min_one_st,
                    nz,
                );
                total_ones + gain
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_gain(
        l: usize,
        r: usize,
        zero_runs: &[(u32, u32, i32)],
        zero_lens: &[i32],
        max_adj_st: &SparseTable,
        max_zero_st: &SparseTable,
        min_one_st: &SparseTable,
        nz: usize,
    ) -> i32 {
        // Locate zero-runs overlapping [l, r].
        let fz = zero_runs.partition_point(|zr| (zr.1 as usize) < l);
        if fz >= nz || (zero_runs[fz].0 as usize) > r {
            return 0;
        }
        let lz = zero_runs
            .partition_point(|zr| (zr.0 as usize) <= r)
            .saturating_sub(1);
        if lz < fz || lz - fz < 1 {
            return 0; // fewer than 2 zero-runs → no valid trade
        }

        // Compute clipped size of a zero-run at index idx within [l, r].
        let clip = |idx: usize| -> i32 {
            let (zs, ze, _) = zero_runs[idx];
            ((ze as usize).min(r) - (zs as usize).max(l) + 1) as i32
        };

        let z_fz = clip(fz);
        let z_lz = clip(lz);

        // Max clipped zero-run size in query (used for Option B).
        let mid_max_zero = if lz > fz + 1 {
            max_zero_st.query(fz + 1, lz - 1)
        } else {
            i32::MIN
        };
        let max_zero_val = z_fz.max(z_lz).max(mid_max_zero);

        let mut max_gain = 0i32;

        // Option A: adjacent pair sacrifice — gain = clipped(z[k]) + clipped(z[k+1]).
        // Boundary pairs (fz and lz) use clipped sizes; interior pairs use the sparse table.

        // Pair at k = fz
        let z_fz1 = if fz + 1 == lz {
            z_lz
        } else {
            zero_lens[fz + 1]
        };
        max_gain = max_gain.max(z_fz + z_fz1);

        // Pair at k = lz - 1 (only if lz > fz, i.e., at least 2 zero-runs)
        let z_lz1 = if lz - 1 == fz {
            z_fz
        } else {
            zero_lens[lz - 1]
        };
        max_gain = max_gain.max(z_lz1 + z_lz);

        // Interior pairs k ∈ [fz+1, lz-2]: both endpoints fully within [l, r].
        if lz >= fz + 3 {
            let interior_adj = max_adj_st.query(fz + 1, lz - 2);
            if interior_adj > i32::MIN {
                max_gain = max_gain.max(interior_adj);
            }
        }

        // Option B: pick best zero-run, sacrifice cheapest inner 1-run.
        // gain = max_zero_val - min_inner_one over [fz, lz-1].
        let min_one_val = min_one_st.query(fz, lz - 1);
        if min_one_val != i32::MAX {
            max_gain = max_gain.max(max_zero_val - min_one_val);
        }

        max_gain
    }
}

/// Sparse table for idempotent range queries (max or min) in O(1) per query.
struct SparseTable {
    table: Vec<Vec<i32>>,
    log: Vec<usize>,
    is_max: bool,
    n: usize,
}

impl SparseTable {
    fn new(arr: &[i32], is_max: bool) -> Self {
        let n = arr.len();
        if n == 0 {
            return Self {
                table: vec![],
                log: vec![],
                is_max,
                n: 0,
            };
        }

        let mut log = vec![0usize; n + 1];
        (2..=n).for_each(|i| log[i] = log[i / 2] + 1);

        let k = log[n] + 1;
        let identity = if is_max { i32::MIN } else { i32::MAX };
        let mut table = vec![vec![identity; n]; k];
        table[0].iter_mut().zip(arr).for_each(|(t, &v)| *t = v);

        for j in 1..k {
            let step = 1 << (j - 1);
            for i in 0..=n.saturating_sub(1 << j) {
                table[j][i] = if is_max {
                    table[j - 1][i].max(table[j - 1][i + step])
                } else {
                    table[j - 1][i].min(table[j - 1][i + step])
                };
            }
        }

        Self {
            table,
            log,
            is_max,
            n,
        }
    }

    fn query(&self, l: usize, r: usize) -> i32 {
        if self.n == 0 || l > r || r >= self.n {
            return if self.is_max { i32::MIN } else { i32::MAX };
        }
        let len = r - l + 1;
        let j = self.log[len];
        let step = 1 << j;
        if self.is_max {
            self.table[j][l].max(self.table[j][r + 1 - step])
        } else {
            self.table[j][l].min(self.table[j][r + 1 - step])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_active_sections_after_trade("01".to_string(), vec![vec![0, 1]]),
            vec![1]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_active_sections_after_trade(
                "0100".to_string(),
                vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]]
            ),
            vec![4, 3, 1, 1]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::max_active_sections_after_trade(
                "1000100".to_string(),
                vec![vec![1, 5], vec![0, 6], vec![0, 4]]
            ),
            vec![6, 7, 2]
        );
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            Solution::max_active_sections_after_trade(
                "01010".to_string(),
                vec![vec![0, 3], vec![1, 4], vec![1, 3]]
            ),
            vec![4, 4, 2]
        );
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(
            Solution::max_active_sections_after_trade("1111".to_string(), vec![vec![0, 3]]),
            vec![4]
        );
    }

    #[test]
    fn test_all_zeros() {
        // No inner 1-run to sacrifice — no valid trade.
        assert_eq!(
            Solution::max_active_sections_after_trade("0000".to_string(), vec![vec![0, 3]]),
            vec![0]
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::max_active_sections_after_trade("0".to_string(), vec![vec![0, 0]]),
            vec![0]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("1".to_string(), vec![vec![0, 0]]),
            vec![1]
        );
    }

    #[test]
    fn test_no_gain_trade() {
        // Query over a window with only one zero-run: no valid trade.
        assert_eq!(
            Solution::max_active_sections_after_trade("1000100".to_string(), vec![vec![0, 4]]),
            vec![2]
        );
    }
}
