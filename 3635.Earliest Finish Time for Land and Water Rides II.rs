impl Solution {
    /// Binary search with prefix/suffix minimums for O((n+m) log m) optimal pairing.
    ///
    /// # Intuition
    /// For every land ride `i`, we try both orderings against every water ride `j`:
    /// - Land then water: `finish = max(lf_i, ws_j) + wd_j`
    /// - Water then land: `finish = max(wf_j, ls_i) + ld_i`
    ///
    /// Brute force is O(n·m) which exceeds the constraint (n, m ≤ 5×10⁴ → 2.5×10⁹ ops).
    /// By sorting water rides and precomputing prefix/suffix minimums we reduce each
    /// land ride's best-pairing query to a binary search, giving O((n+m) log m) overall.
    ///
    /// # Approach
    /// **Land-first preprocessing** — sort water by start time `ws`:
    /// - Case A (`ws_j >= lf_i`): `finish = ws_j + wd_j` → suffix-min of `ws+wd` from split point
    /// - Case B (`ws_j <  lf_i`): `finish = lf_i + wd_j` → `lf_i` + prefix-min of `wd` up to split
    ///
    /// **Water-first preprocessing** — sort water by finish time `wf`:
    /// - Case C (`wf_j <= ls_i`): `finish = ls_i + ld_i = lf_i` → any such ride suffices
    /// - Case D (`wf_j >  ls_i`): `finish = wf_j + ld_i` → suffix-min of `wf` from split point
    ///
    /// For each land ride, binary-search both sorted arrays to find split points, then
    /// combine the four cases and track the global minimum.
    ///
    /// # Complexity
    /// - Time: O((n + m) log m)
    /// - Space: O(m)
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let m = water_start_time.len();

        // ── water rides sorted by start time ─────────────────────────────────
        let mut by_start: Vec<(i32, i32)> = water_start_time
            .iter()
            .zip(water_duration.iter())
            .map(|(&s, &d)| (s, d))
            .collect();
        by_start.sort_unstable();

        // prefix_min_wd[k] = min waterDuration among by_start[0..=k]
        let mut prefix_min_wd = Vec::with_capacity(m);
        let mut running = i32::MAX;
        for &(_, wd) in &by_start {
            running = running.min(wd);
            prefix_min_wd.push(running);
        }

        // suffix_min_total[k] = min (ws + wd) among by_start[k..]
        let mut suffix_min_total = vec![i32::MAX; m + 1];
        for k in (0..m).rev() {
            let (ws, wd) = by_start[k];
            suffix_min_total[k] = suffix_min_total[k + 1].min(ws + wd);
        }

        // ── water rides sorted by finish time ────────────────────────────────
        let mut by_finish: Vec<i32> = water_start_time
            .iter()
            .zip(water_duration.iter())
            .map(|(&s, &d)| s + d)
            .collect();
        by_finish.sort_unstable();

        // suffix_min_wf[k] = min finish time among by_finish[k..]
        let mut suffix_min_wf = vec![i32::MAX; m + 1];
        for k in (0..m).rev() {
            suffix_min_wf[k] = suffix_min_wf[k + 1].min(by_finish[k]);
        }

        // ── iterate land rides, query precomputed arrays ──────────────────────
        let mut ans = i32::MAX;

        for (&ls, &ld) in land_start_time.iter().zip(land_duration.iter()) {
            let lf = ls + ld;

            // Land first, then water: finish = max(lf, ws_j) + wd_j
            let split_a = by_start.partition_point(|&(ws, _)| ws < lf);
            let case_a = suffix_min_total[split_a]; // ws_j >= lf branch
            let case_b = if split_a > 0 {
                lf + prefix_min_wd[split_a - 1]
            } else {
                i32::MAX
            };
            let land_first = case_a.min(case_b);

            // Water first, then land: finish = max(wf_j, ls) + ld
            let split_c = by_finish.partition_point(|&wf| wf <= ls);
            let case_c = if split_c > 0 { lf } else { i32::MAX }; // wf_j <= ls → finish = lf
            let case_d = if split_c < m {
                suffix_min_wf[split_c] + ld
            } else {
                i32::MAX
            };
            let water_first = case_c.min(case_d);

            ans = ans.min(land_first).min(water_first);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Plan A: land0(finish=6) → water0(start=6): max(6,6)+3 = 9
        assert_eq!(
            Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]),
            9
        );
    }

    #[test]
    fn test_example_2() {
        // water0(finish=11) → land0(start=5): max(11,5)+3 = 14
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]),
            14
        );
    }

    #[test]
    fn test_land_first_no_wait() {
        // land: finish=3, water starts at 3 → max(3,3)+1 = 4
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![2], vec![3], vec![1]),
            4
        );
    }

    #[test]
    fn test_water_first_wait_for_land() {
        // water finish=7, land opens at 10 → max(7,10)+1 = 11
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![5], vec![2]),
            11
        );
    }

    #[test]
    fn test_multiple_rides_pick_best() {
        // land(1,1) finish=2, water(2,1) start=2 → max(2,2)+1 = 3  (best)
        assert_eq!(
            Solution::earliest_finish_time(vec![1, 3], vec![1, 1], vec![2, 5], vec![1, 1]),
            3
        );
    }

    #[test]
    fn test_water_long_before_land() {
        // water finish=2, land opens at 10 → max(2,10)+1 = 11
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![1], vec![1]),
            11
        );
    }

    #[test]
    fn test_water_opens_after_land_finishes() {
        // land finish=2, water starts at 5 → max(2,5)+1 = 6
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![1], vec![5], vec![1]),
            6
        );
    }

    #[test]
    fn test_same_start_time() {
        // land(5,3) finish=8, water(5,2) finish=7
        // land→water: ws=5 < lf=8 → case B: 8+2=10; case A: none → 10
        // water→land: wf=7 > ls=5 → case D: 7+3=10 → 10
        assert_eq!(
            Solution::earliest_finish_time(vec![5], vec![3], vec![5], vec![2]),
            10
        );
    }

    #[test]
    fn test_boundary_max_duration() {
        // Both start at 1, both duration 100000
        // land→water: lf=100001, ws=1 < lf → 100001+100000 = 200001
        // water→land: wf=100001 > ls=1 → 100001+100000 = 200001
        assert_eq!(
            Solution::earliest_finish_time(vec![1], vec![100000], vec![1], vec![100000]),
            200001
        );
    }

    #[test]
    fn test_many_water_pick_shortest() {
        // land: start=10, dur=1, finish=11
        // water: [(1,5,finish=6), (2,3,finish=5), (3,1,finish=4)]
        // water→land: all finish before ls=10 → case C: lf=11
        // land→water: all ws < lf=11 → case B: 11 + min_wd = 11+1 = 12
        // best = 11
        assert_eq!(
            Solution::earliest_finish_time(vec![10], vec![1], vec![1, 2, 3], vec![5, 3, 1]),
            11
        );
    }
}
