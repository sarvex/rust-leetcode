impl Solution {
    /// Two-pointer monotonic sweep — O((n+m) log(n+m)) with minimal allocations.
    ///
    /// # Intuition
    /// For each land ride paired with each water ride, finish time depends on order:
    /// - Land then water: `finish = max(lf, ws) + wd`
    /// - Water then land: `finish = max(wf, ls) + ld`
    ///
    /// Sort both land and water rides, then use monotonic pointers to find best pairing
    /// instead of binary searching per land ride.
    ///
    /// # Approach
    /// **Preprocessing water rides:**
    /// - Sort by start time → compute prefix-min of duration, suffix-min of (start+duration)
    /// - Sort by finish time → compute suffix-min of finish time
    ///
    /// **Two sweeps over land rides:**
    /// 1. Sort land by finish time, sweep water start pointer → land-first cases
    /// 2. Sort land by start time, sweep water finish pointer → water-first cases
    ///
    /// # Complexity
    /// - Time: O((n+m) log(n+m))
    /// - Space: O(n+m)
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let n = land_start_time.len();
        let m = water_start_time.len();

        // ── Water sorted by start time ────────────────────────────────────────
        let mut ws_idx: Vec<usize> = (0..m).collect();
        ws_idx.sort_unstable_by_key(|&i| water_start_time[i]);

        let mut prefix_wd = vec![i32::MAX; m];
        prefix_wd[0] = water_duration[ws_idx[0]];
        for k in 1..m {
            prefix_wd[k] = prefix_wd[k - 1].min(water_duration[ws_idx[k]]);
        }

        let mut suffix_sum = vec![i32::MAX; m + 1];
        for k in (0..m).rev() {
            let i = ws_idx[k];
            suffix_sum[k] = suffix_sum[k + 1].min(water_start_time[i] + water_duration[i]);
        }

        // ── Water sorted by finish time ───────────────────────────────────────
        let mut wf_idx: Vec<usize> = (0..m).collect();
        wf_idx.sort_unstable_by_key(|&i| water_start_time[i] + water_duration[i]);

        let mut suffix_wf = vec![i32::MAX; m + 1];
        for k in (0..m).rev() {
            let i = wf_idx[k];
            suffix_wf[k] = suffix_wf[k + 1].min(water_start_time[i] + water_duration[i]);
        }

        let mut ans = vec![i32::MAX; n];

        // ── Sweep 1: land-first (sort land by finish time) ───────────────────
        let mut lf_order: Vec<usize> = (0..n).collect();
        lf_order.sort_unstable_by_key(|&i| land_start_time[i] + land_duration[i]);

        let mut p = 0;
        for &i in &lf_order {
            let lf = land_start_time[i] + land_duration[i];
            // advance p to first water ride where ws >= lf
            while p < m && water_start_time[ws_idx[p]] < lf {
                p += 1;
            }
            let a = suffix_sum[p]; // ws >= lf → finish = ws + wd
            let b = if p > 0 {
                lf + prefix_wd[p - 1]
            } else {
                i32::MAX
            }; // ws < lf
            ans[i] = ans[i].min(a.min(b));
        }

        // ── Sweep 2: water-first (sort land by start time) ───────────────────
        let mut ls_order: Vec<usize> = (0..n).collect();
        ls_order.sort_unstable_by_key(|&i| land_start_time[i]);

        let mut q = 0;
        for &i in &ls_order {
            let ls = land_start_time[i];
            let ld = land_duration[i];
            let lf = ls + ld;
            // advance q to first water ride where wf > ls
            while q < m {
                let wf = water_start_time[wf_idx[q]] + water_duration[wf_idx[q]];
                if wf > ls {
                    break;
                }
                q += 1;
            }
            let c = if q > 0 { lf } else { i32::MAX }; // wf <= ls
            let d = if q < m { suffix_wf[q] + ld } else { i32::MAX }; // wf > ls
            ans[i] = ans[i].min(c.min(d));
        }

        *ans.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]),
            14
        );
    }

    #[test]
    fn test_land_first_no_wait() {
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![2], vec![3], vec![1]),
            4
        );
    }

    #[test]
    fn test_water_first_wait_for_land() {
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![5], vec![2]),
            11
        );
    }

    #[test]
    fn test_multiple_rides_pick_best() {
        assert_eq!(
            Solution::earliest_finish_time(vec![1, 3], vec![1, 1], vec![2, 5], vec![1, 1]),
            3
        );
    }

    #[test]
    fn test_water_long_before_land() {
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![1], vec![1]),
            11
        );
    }

    #[test]
    fn test_water_opens_after_land_finishes() {
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![1], vec![5], vec![1]),
            6
        );
    }

    #[test]
    fn test_same_start_time() {
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![5], vec![2]),
            10
        );
    }

    #[test]
    fn test_boundary_max_duration() {
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![100000], vec![1], vec![100000]),
            200001
        );
    }

    #[test]
    fn test_many_water_pick_shortest() {
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![1, 2, 3], vec![5, 3, 1]),
            11
        );
    }

    #[test]
    fn test_many_land_many_water() {
        assert_eq!(
            Solution::earliest_finish_time(
                vec![1, 3, 5],
                vec![2, 1, 3],
                vec![2, 4, 6],
                vec![1, 2, 1]
            ),
            4
        );
    }
}
