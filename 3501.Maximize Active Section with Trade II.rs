impl Solution {
    /// Uses sparse tables over 0-runs for O(log n) query processing.
    ///
    /// # Intuition
    /// Trade: sacrifice inner 1-run, convert 0-run to 1's.
    /// Option 1: merge adjacent 0-runs (gain = sum)
    /// Option 2: sacrifice min 1-run, gain max non-adjacent 0-run
    ///
    /// # Approach
    /// 1. Precompute runs and separate 0-runs from 1-runs
    /// 2. Build sparse tables for range max of adjacent sums, max 0-run, min 1-run
    /// 3. For each query, use binary search to find relevant 0-run range
    /// 4. Handle partial boundary 0-runs, use sparse tables for middle
    ///
    /// # Complexity
    /// - Time: O(n log n + q log n)
    /// - Space: O(n)
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();

        let total_ones: i32 = s.iter().filter(|&&c| c == b'1').count() as i32;

        // Build runs: (start, end, is_one)
        let mut runs: Vec<(u32, u32, bool)> = Vec::new();
        let mut i = 0u32;
        while (i as usize) < n {
            let start = i;
            let is_one = s[i as usize] == b'1';
            while (i as usize) < n && (s[i as usize] == b'1') == is_one {
                i += 1;
            }
            runs.push((start, i - 1, is_one));
        }

        if runs.is_empty() {
            return queries.iter().map(|_| total_ones).collect();
        }

        // Extract 0-runs with their positions: (run_start, run_end, length)
        let zero_runs: Vec<(u32, u32, i32)> = runs
            .iter()
            .filter(|r| !r.2)
            .map(|r| (r.0, r.1, (r.1 - r.0 + 1) as i32))
            .collect();

        let nz = zero_runs.len();
        if nz == 0 {
            return queries.iter().map(|_| total_ones).collect();
        }

        // zero_lens for sparse tables
        let zero_lens: Vec<i32> = zero_runs.iter().map(|r| r.2).collect();

        // Adjacent sums: adj_sums[i] = zero_lens[i] + zero_lens[i+1]
        let adj_sums: Vec<i32> = if nz >= 2 {
            zero_lens.windows(2).map(|w| w[0] + w[1]).collect()
        } else {
            vec![]
        };

        // Inner 1-runs: 1-run between consecutive 0-runs
        // inner_ones[i] = total length of 1-runs between 0-run i and i+1
        let inner_ones: Vec<i32> = if nz >= 2 {
            (0..nz - 1)
                .map(|i| {
                    let end_of_i = zero_runs[i].1;
                    let start_of_next = zero_runs[i + 1].0;
                    // Length of 1-runs between them
                    (start_of_next - end_of_i - 1) as i32
                })
                .collect()
        } else {
            vec![]
        };

        // Build sparse tables
        let max_adj_st = SparseTable::new_max(&adj_sums);
        let max_zero_st = SparseTable::new_max(&zero_lens);
        let min_one_st = SparseTable::new_min(&inner_ones);

        queries
            .iter()
            .map(|q| {
                let l = q[0] as u32;
                let r = q[1] as u32;

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
        l: u32,
        r: u32,
        zero_runs: &[(u32, u32, i32)],
        zero_lens: &[i32],
        max_adj_st: &SparseTable,
        max_zero_st: &SparseTable,
        min_one_st: &SparseTable,
        nz: usize,
    ) -> i32 {
        if nz == 0 {
            return 0;
        }

        // Find first 0-run that overlaps with [l, r]
        // Binary search: find first 0-run where end >= l
        let first_zero = zero_runs.partition_point(|zr| zr.1 < l);
        if first_zero >= nz {
            return 0; // No 0-run overlaps
        }

        // Check if this 0-run actually overlaps (start <= r)
        if zero_runs[first_zero].0 > r {
            return 0;
        }

        // Find last 0-run that overlaps with [l, r]
        // Binary search: find first 0-run where start > r, then go back one
        let last_zero = zero_runs.partition_point(|zr| zr.0 <= r).saturating_sub(1);

        if last_zero < first_zero {
            return 0;
        }

        let fz = first_zero;
        let lz = last_zero;
        let num_zeros = lz - fz + 1;

        if num_zeros <= 1 {
            return 0;
        }

        // Compute effective lengths for boundary 0-runs
        let first_zero_eff = {
            let (zs, ze, _) = zero_runs[fz];
            let os = zs.max(l);
            let oe = ze.min(r);
            (oe - os + 1) as i32
        };

        let last_zero_eff = if fz == lz {
            first_zero_eff
        } else {
            let (zs, ze, _) = zero_runs[lz];
            let os = zs.max(l);
            let oe = ze.min(r);
            (oe - os + 1) as i32
        };

        let mut max_gain = 0i32;

        // Option 1: max sum of adjacent 0-runs
        if num_zeros >= 2 {
            // Compute effective length of second 0-run (fz+1)
            let second_eff = if fz + 1 == lz {
                last_zero_eff // It's also the last, use its effective length
            } else {
                // It's fully included (not the last, and not the first)
                zero_lens[fz + 1]
            };

            // First pair: [fz, fz+1]
            max_gain = max_gain.max(first_zero_eff + second_eff);

            // Last pair: [lz-1, lz]
            if lz > fz + 1 {
                // second_last is fully included
                let second_last = zero_lens[lz - 1];
                max_gain = max_gain.max(second_last + last_zero_eff);
            }

            // Middle pairs: all fully included, use sparse table
            if lz >= fz + 3 {
                // Pairs from fz+1 to lz-2 (adj_sums indices)
                let adj_max = max_adj_st.query(fz + 1, lz - 2);
                max_gain = max_gain.max(adj_max);
            }
        }

        // Option 2: max 0-run - min inner 1-run (for non-adjacent)
        if num_zeros >= 3 {
            // Max 0-run: consider boundary effective lengths and middle
            let mid_max = if lz > fz + 1 {
                max_zero_st.query(fz + 1, lz - 1)
            } else {
                i32::MIN
            };
            let max_zero_val = first_zero_eff.max(last_zero_eff).max(mid_max);

            // Min inner 1-run: between 0-runs in range
            // inner_ones[i] is between 0-run i and i+1
            // We need inner_ones[fz..lz-1] for 1-runs fully between our 0-runs
            let min_one_val = if lz > fz && fz < min_one_st.len() {
                min_one_st.query(fz, (lz - 1).min(min_one_st.len() - 1))
            } else {
                i32::MAX
            };

            if max_zero_val != i32::MIN && min_one_val != i32::MAX {
                max_gain = max_gain.max(max_zero_val - min_one_val);
            }
        }

        max_gain
    }
}

struct SparseTable {
    table: Vec<Vec<i32>>,
    log: Vec<usize>,
    is_max: bool,
    n: usize,
}

impl SparseTable {
    fn new_max(arr: &[i32]) -> Self {
        Self::new(arr, true)
    }

    fn new_min(arr: &[i32]) -> Self {
        Self::new(arr, false)
    }

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
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }

        let k = log[n] + 1;
        let identity = if is_max { i32::MIN } else { i32::MAX };
        let mut table = vec![vec![identity; n]; k];

        for (i, &v) in arr.iter().enumerate() {
            table[0][i] = v;
        }

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

    fn len(&self) -> usize {
        self.n
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
        let s = "01".to_string();
        let queries = vec![vec![0, 1]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
            vec![1]
        );
    }

    #[test]
    fn test_example_2() {
        let s = "0100".to_string();
        let queries = vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
            vec![4, 3, 1, 1]
        );
    }

    #[test]
    fn test_example_3() {
        let s = "1000100".to_string();
        let queries = vec![vec![1, 5], vec![0, 6], vec![0, 4]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
            vec![6, 7, 2]
        );
    }

    #[test]
    fn test_example_4() {
        let s = "01010".to_string();
        let queries = vec![vec![0, 3], vec![1, 4], vec![1, 3]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
            vec![4, 4, 2]
        );
    }

    #[test]
    fn test_all_ones() {
        let s = "1111".to_string();
        let queries = vec![vec![0, 3]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
            vec![4]
        );
    }

    #[test]
    fn test_all_zeros() {
        let s = "0000".to_string();
        let queries = vec![vec![0, 3]];
        assert_eq!(
            Solution::max_active_sections_after_trade(s, queries),
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
}
