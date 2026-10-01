impl Solution {
    /// Sliding window + prefix minimum of best "left half" subarray length.
    ///
    /// # Intuition
    /// Because every element satisfies `arr[i] >= 1`, prefix sums are strictly
    /// increasing, so a classic two-pointer sliding window identifies **all**
    /// subarrays with sum equal to `target` in a single left-to-right pass.
    ///
    /// For every such subarray `[l..=r]` found, we want to pair it with the
    /// shortest *other* subarray that lies entirely to its left (ending at
    /// index `< l`). If we maintain, for each 1-based position `i`, the
    /// minimum length of any target-sum subarray contained in `arr[0..i]`,
    /// then at the moment we discover `[l..=r]` the best partner is simply
    /// `best[l]` (which refers to `arr[0..l]`).
    ///
    /// Taking the minimum of `(r - l + 1) + best[l]` over all discovered
    /// windows yields the answer in O(n) time.
    ///
    /// # Approach
    /// 1. Walk `right` across the slice, advancing `left` whenever the window
    ///    sum exceeds `target` (valid because all elements are positive).
    /// 2. Carry the prefix minimum forward: `best[right + 1] = best[right]`.
    /// 3. When the window sum equals `target`, with `len = right - left + 1`:
    ///    * Partner length is `best[left]`; if finite, update the answer with
    ///      `len + best[left]`.
    ///    * Tighten the prefix minimum: if `len < best[right + 1]`, overwrite.
    /// 4. Return the final answer, or `-1` if no pair was ever formed.
    ///
    /// # Constant-factor tuning
    /// * All arithmetic stays in `i32`: given `arr[i] <= 1000` and
    ///   `target <= 10^8`, the window sum never exceeds `target + 1000`, which
    ///   is well under `i32::MAX`.
    /// * Shifting `best` by `+1` eliminates the two "boundary" branches
    ///   (`right == 0` and `left > 0`) that would otherwise live inside the
    ///   hot loop.
    /// * Binding `arr` as a slice lets the compiler elide bounds checks when
    ///   iterating by index.
    ///
    /// # Complexity
    /// - Time: O(n) — each index enters and leaves the window at most once.
    /// - Space: O(n) for the prefix-minimum array.
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        const INF: i32 = i32::MAX;

        // `best[i]` = min length of any target-sum subarray fully contained in
        // `arr[0..i]`. `best[0] = INF` means "nothing available before index 0".
        let mut best = vec![INF; n + 1];
        let a = arr.as_slice();
        let mut answer = INF;
        let mut left = 0usize;
        let mut window_sum: i32 = 0;

        for right in 0..n {
            window_sum += a[right];
            while window_sum > target {
                window_sum -= a[left];
                left += 1;
            }

            // Default: carry the prefix minimum forward.
            let mut cur_best = best[right];

            if window_sum == target {
                let len = (right - left + 1) as i32;
                let partner = best[left];
                if partner != INF && len + partner < answer {
                    answer = len + partner;
                }
                if len < cur_best {
                    cur_best = len;
                }
            }
            best[right + 1] = cur_best;
        }

        if answer == INF {
            -1
        } else {
            answer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(
            Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
            -1
        );
    }

    #[test]
    fn test_no_subarray_matches() {
        assert_eq!(Solution::min_sum_of_lengths(vec![1, 1, 1, 1], 5), -1);
    }

    #[test]
    fn test_only_one_match() {
        // Single occurrence of target sum — need two, so answer is -1.
        assert_eq!(Solution::min_sum_of_lengths(vec![5, 1, 1, 1, 1], 5), -1);
    }

    #[test]
    fn test_two_adjacent_singletons() {
        assert_eq!(Solution::min_sum_of_lengths(vec![5, 5, 5], 5), 2);
    }

    #[test]
    fn test_overlapping_candidates_must_be_rejected() {
        // Target 8 can be formed by [3,5], [5,3], [3,5] but they overlap.
        // Non-overlapping choices: [3,5] (idx 0..=1) and [5,3] (idx 2..=3) → length 4,
        // or [3,5] (idx 0..=1) and [3,5] (idx 4..=5) → length 4.
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 5, 5, 3, 3, 5], 8), 4);
    }

    #[test]
    fn test_prefers_shorter_pair_across_many_candidates() {
        // Target 7: candidates [7] at 0, [3,4] at 1..=2, [7] at 3.
        // Best pair is the two singletons → length 2 (skipping the medial 2-length one).
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    }

    #[test]
    fn test_long_run_of_ones() {
        // arr = [1;10], target = 3 → every window of length 3 sums to 3.
        // Best two non-overlapping windows each have length 3 → total 6.
        assert_eq!(Solution::min_sum_of_lengths(vec![1; 10], 3), 6);
    }

    #[test]
    fn test_single_element_cannot_pair() {
        assert_eq!(Solution::min_sum_of_lengths(vec![5], 5), -1);
    }

    #[test]
    fn test_large_target_requires_full_prefix_and_suffix() {
        // target = 10, arr = [1,1,1,1,10,1,1,1,1,10]
        // Candidates: [1,1,1,1,...] prefix sums reach 10 at length 10 only w/ the 10,
        // and singletons [10] at index 4 and index 9.
        // Best = singletons → 2.
        assert_eq!(
            Solution::min_sum_of_lengths(vec![1, 1, 1, 1, 10, 1, 1, 1, 1, 10], 10),
            2
        );
    }

    #[test]
    fn test_requires_choosing_non_minimum_first_window() {
        // target = 6
        // arr = [1,6,1,1,1,1,1,1]
        // Candidates: [6] at idx 1 (len 1), [1,1,1,1,1,1] at idx 2..=7 (len 6).
        // Only pair possible: 1 + 6 = 7.
        assert_eq!(
            Solution::min_sum_of_lengths(vec![1, 6, 1, 1, 1, 1, 1, 1], 6),
            7
        );
    }

    #[test]
    fn test_stress_monotone_increasing() {
        // target = 15 in [1..=5,1..=5]: subarrays summing to 15 → full [1..=5] twice.
        assert_eq!(
            Solution::min_sum_of_lengths(vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5], 15),
            10
        );
    }
}
