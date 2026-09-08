impl Solution {
    /// Prefix-maximum and suffix-minimum sweep to find the first stable index.
    ///
    /// # Intuition
    /// The instability score at index `i` is
    /// `max(nums[0..=i]) - min(nums[i..=n-1])`. The prefix maximum is
    /// non-decreasing as `i` grows and the suffix minimum is non-increasing,
    /// so the score never *decreases* on its own — but we still need every
    /// index checked because both endpoints matter. Both windows include index
    /// `i` itself, so the two quantities can be maintained with a single
    /// forward-running maximum and a precomputed suffix minimum.
    ///
    /// # Approach
    /// Build a suffix-minimum array where `suffix_min[i] = min(nums[i..n])`.
    /// Then sweep `i` from `0` to `n - 1`, tracking the running prefix maximum
    /// `max(nums[0..=i])`. At each `i` the score is
    /// `prefix_max - suffix_min[i]`; return the first index whose score is
    /// `<= k`. If none qualifies, return `-1`.
    ///
    /// Using `i64` for the subtraction avoids overflow since values reach
    /// `10^9` and the difference is compared against `k` up to `10^9`.
    ///
    /// # Complexity
    /// - Time: O(n) — one pass to build suffix minimums, one pass to scan
    /// - Space: O(n) — suffix-minimum array
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;

        let mut suffix_min = vec![0i32; n];
        suffix_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            suffix_min[i] = nums[i].min(suffix_min[i + 1]);
        }

        let mut prefix_max = i32::MIN;
        for i in 0..n {
            prefix_max = prefix_max.max(nums[i]);
            if prefix_max as i64 - suffix_min[i] as i64 <= k {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::first_stable_index(vec![3, 2, 1], 1), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::first_stable_index(vec![0], 0), 0);
    }

    #[test]
    fn first_index_stable() {
        // Non-decreasing array: index 0 already stable when k is large enough.
        assert_eq!(Solution::first_stable_index(vec![1, 2, 3, 4], 0), 0);
    }

    #[test]
    fn large_values_no_overflow() {
        // Difference of 10^9 with k just below it fails, at k it succeeds.
        assert_eq!(
            Solution::first_stable_index(vec![1_000_000_000, 0], 999_999_999),
            -1
        );
        assert_eq!(
            Solution::first_stable_index(vec![1_000_000_000, 0], 1_000_000_000),
            0
        );
    }

    #[test]
    fn all_equal_elements() {
        // Every score is 0, so the first index is stable for any k >= 0.
        assert_eq!(Solution::first_stable_index(vec![7, 7, 7, 7], 0), 0);
    }

    #[test]
    fn stability_only_at_end() {
        // Score decreases toward the end as the suffix min rises.
        assert_eq!(Solution::first_stable_index(vec![9, 1, 2, 8], 1), 3);
    }
}
