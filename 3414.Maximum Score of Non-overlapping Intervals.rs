impl Solution {
    /// Maximum Score of Non-overlapping Intervals
    ///
    /// # Intuition
    /// Select up to 4 non-overlapping intervals to maximize total weight. When multiple
    /// solutions have equal weight, return lexicographically smallest indices.
    ///
    /// # Approach
    /// 1. Sort intervals by end position while preserving original indices
    /// 2. Use DP where dp[k][i] tracks (max_weight, sorted_indices) for exactly k intervals
    /// 3. For each interval, binary search finds the latest non-overlapping predecessor
    /// 4. Track lexicographically smallest indices when weights are equal
    /// 5. Return the best solution among 1 to 4 intervals
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting and binary search
    /// - Space: O(n) for DP table with O(1) index storage per cell
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let mut indexed: Vec<(i64, i64, i64, usize)> = intervals
            .iter()
            .enumerate()
            .map(|(i, v)| (v[0] as i64, v[1] as i64, v[2] as i64, i))
            .collect();

        indexed.sort_by_key(|x| x.1);

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
                    let prev = dp[k][i - 1].clone();
                    dp[k][i] = prev;
                }

                let prev_state = if k == 1 {
                    Some((0i64, vec![]))
                } else if prev >= 0 {
                    dp[k - 1][prev as usize].clone()
                } else {
                    None
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
    fn test_example_1() {
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
    fn test_example_2() {
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
    fn test_single_interval() {
        let intervals = vec![vec![1, 5, 10]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0]);
    }

    #[test]
    fn test_all_overlapping() {
        let intervals = vec![vec![1, 10, 5], vec![2, 8, 3], vec![3, 7, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0]);
    }

    #[test]
    fn test_non_overlapping_chain() {
        let intervals = vec![vec![1, 2, 1], vec![3, 4, 2], vec![5, 6, 3], vec![7, 8, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_lexicographic_tiebreaker() {
        let intervals = vec![vec![1, 2, 5], vec![3, 4, 5], vec![1, 2, 5], vec![3, 4, 5]];
        let result = Solution::maximum_weight(intervals);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_boundary_non_overlap() {
        let intervals = vec![vec![1, 5, 3], vec![5, 10, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![1]);
    }

    #[test]
    fn test_strict_non_overlap() {
        let intervals = vec![vec![1, 4, 3], vec![5, 10, 4]];
        assert_eq!(Solution::maximum_weight(intervals), vec![0, 1]);
    }
}
