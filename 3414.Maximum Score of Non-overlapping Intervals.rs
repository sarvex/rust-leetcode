impl Solution {
    /// Selects up to 4 non-overlapping intervals maximizing total weight.
    ///
    /// # Intuition
    /// Sort intervals by end position and apply DP over the number of intervals
    /// selected (1–4). Binary search finds the latest non-overlapping predecessor.
    /// Ties in weight are broken by lexicographically smallest index vector.
    ///
    /// # Approach
    /// 1. Sort by end position, preserving original indices.
    /// 2. For each interval and each k in 1..=4, binary-search the latest
    ///    predecessor and extend dp[k-1][pred] by the current interval.
    /// 3. Propagate the better of "skip" vs "take" per cell.
    /// 4. Return the best across k = 1..=4.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) per DP layer
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let mut indexed: Vec<(i64, i64, i64, usize)> = intervals
            .iter()
            .enumerate()
            .map(|(i, v)| (v[0] as i64, v[1] as i64, v[2] as i64, i))
            .collect();

        indexed.sort_unstable_by_key(|x| x.1);

        let find_prev = |target: i64, end: usize| -> i32 {
            let (mut lo, mut hi) = (-1i32, end as i32 - 1);
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if indexed[mid as usize].1 < target {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            lo
        };

        type State = Option<(i64, Vec<i32>)>;
        let mut dp: Vec<Vec<State>> = vec![vec![None; n]; 5];

        let is_better = |current: &State, candidate: &(i64, Vec<i32>)| -> bool {
            match current {
                None => true,
                Some((w, indices)) => {
                    candidate.0 > *w || (candidate.0 == *w && candidate.1 < *indices)
                }
            }
        };

        for i in 0..n {
            let (left, _, weight, orig_idx) = indexed[i];
            let prev = find_prev(left, i);

            for k in 1..=4usize {
                if i > 0 {
                    let prev_state = dp[k][i - 1].clone();
                    dp[k][i] = prev_state;
                }

                let prev_state = match k {
                    1 => Some((0i64, vec![])),
                    _ if prev >= 0 => dp[k - 1][prev as usize].clone(),
                    _ => None,
                };

                if let Some((prev_weight, mut prev_indices)) = prev_state {
                    let new_weight = prev_weight + weight;
                    prev_indices.push(orig_idx as i32);
                    prev_indices.sort_unstable();
                    let candidate = (new_weight, prev_indices);

                    if is_better(&dp[k][i], &candidate) {
                        dp[k][i] = Some(candidate);
                    }
                }
            }
        }

        (1..=4)
            .filter_map(|k| dp[k][n - 1].as_ref())
            .max_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)))
            .map(|(_, indices)| indices.clone())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_two_non_overlapping_intervals() {
        let intervals = vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1],
        ];
        assert_eq!(Solution::maximum_weight(intervals), vec![2, 3]);
    }

    #[test]
    fn selects_four_non_overlapping_intervals() {
        let intervals = vec![
            vec![5, 8, 1],
            vec![6, 7, 7],
            vec![4, 7, 3],
            vec![9, 10, 6],
            vec![7, 8, 2],
            vec![11, 14, 3],
            vec![3, 5, 5],
        ];
        assert_eq!(Solution::maximum_weight(intervals), vec![1, 3, 5, 6]);
    }

    #[test]
    fn single_interval_returns_its_index() {
        assert_eq!(Solution::maximum_weight(vec![vec![1, 5, 10]]), vec![0]);
    }

    #[test]
    fn all_overlapping_picks_heaviest() {
        let intervals = vec![vec![1, 10, 5], vec![2, 8, 3], vec![3, 7, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0]);
    }

    #[test]
    fn non_overlapping_chain_selects_all_four() {
        let intervals = vec![vec![1, 2, 1], vec![3, 4, 2], vec![5, 6, 3], vec![7, 8, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0, 1, 2, 3]);
    }

    #[test]
    fn lexicographic_tiebreaker_selects_smallest_indices() {
        let intervals = vec![vec![1, 2, 5], vec![3, 4, 5], vec![1, 2, 5], vec![3, 4, 5]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0, 1]);
    }

    #[test]
    fn boundary_overlap_not_non_overlapping() {
        assert_eq!(
            Solution::maximum_weight(vec![vec![1, 5, 3], vec![5, 10, 4]]),
            vec![1]
        );
    }

    #[test]
    fn strict_non_overlap_selects_both() {
        assert_eq!(
            Solution::maximum_weight(vec![vec![1, 4, 3], vec![5, 10, 4]]),
            vec![0, 1]
        );
    }
}
