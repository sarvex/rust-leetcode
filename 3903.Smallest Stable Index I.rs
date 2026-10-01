impl Solution {
    /// Smallest index whose prefix-max minus suffix-min is within `k`.
    ///
    /// # Intuition
    /// The instability score at index `i` is `max(nums[0..=i]) -
    /// min(nums[i..n])`. Both halves are monotone as `i` moves — the
    /// prefix max never decreases and the suffix min never increases —
    /// yet their difference is **not** monotone, so we cannot binary
    /// search. We can still answer in a single linear pass by streaming
    /// the prefix max while reading a precomputed suffix min.
    ///
    /// # Approach
    /// 1. Build `suffix_min` where `suffix_min[i] = min(nums[i..n])` with
    ///    one right-to-left scan (functional `scan` over the reversed
    ///    iterator, then reversed back).
    /// 2. Walk left-to-right, folding `prefix_max` with another `scan`
    ///    and zipping against `suffix_min`.
    /// 3. Return the first position whose gap `prefix_max - suffix_min`
    ///    is `<= k`; `-1` if none exists.
    ///
    /// # Complexity
    /// - Time: O(n) — two linear passes.
    /// - Space: O(n) — the suffix-min buffer.
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let suffix_min: Vec<i32> = {
            let mut tail: Vec<i32> = nums
                .iter()
                .rev()
                .scan(i32::MAX, |acc, &x| {
                    *acc = (*acc).min(x);
                    Some(*acc)
                })
                .collect();
            tail.reverse();
            tail
        };

        nums.iter()
            .scan(i32::MIN, |acc, &x| {
                *acc = (*acc).max(x);
                Some(*acc)
            })
            .zip(suffix_min.iter())
            .position(|(prefix_max, &suffix_min)| prefix_max - suffix_min <= k)
            .map_or(-1, |i| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::first_stable_index(vec![5, 0, 1, 4], 3), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::first_stable_index(vec![3, 2, 1], 1), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::first_stable_index(vec![0], 0), 0);
    }

    #[test]
    fn single_element_large_value() {
        // One element: score is always 0, so index 0 is stable for any k >= 0.
        assert_eq!(Solution::first_stable_index(vec![1_000_000_000], 0), 0);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::first_stable_index(vec![7, 7, 7, 7], 0), 0);
    }

    #[test]
    fn strictly_increasing_k_zero() {
        // nums = [1,2,3,4,5]; score at i: max(0..=i) - min(i..n) = nums[i] - nums[i] = 0.
        assert_eq!(Solution::first_stable_index(vec![1, 2, 3, 4, 5], 0), 0);
    }

    #[test]
    fn strictly_decreasing_tight_k() {
        // nums = [5,4,3,2,1]; score at every i is 5 - 1 = 4.
        assert_eq!(Solution::first_stable_index(vec![5, 4, 3, 2, 1], 3), -1);
        assert_eq!(Solution::first_stable_index(vec![5, 4, 3, 2, 1], 4), 0);
    }

    #[test]
    fn first_stable_in_middle() {
        // nums = [10, 1, 2, 100]
        // i=0: max=10, min=1 -> 9
        // i=1: max=10, min=1 -> 9
        // i=2: max=10, min=2 -> 8
        // i=3: max=100, min=100 -> 0
        assert_eq!(Solution::first_stable_index(vec![10, 1, 2, 100], 8), 2);
        assert_eq!(Solution::first_stable_index(vec![10, 1, 2, 100], 0), 3);
    }

    #[test]
    fn large_k_always_stable() {
        assert_eq!(
            Solution::first_stable_index(vec![0, 1_000_000_000, 0], 1_000_000_000),
            0
        );
    }

    #[test]
    fn boundary_values() {
        // Mix of extremes to exercise i32 arithmetic without overflow risk.
        let nums = vec![1_000_000_000, 0, 1_000_000_000, 0];
        // i=0: max=1e9, min=0 -> 1e9
        // i=1: max=1e9, min=0 -> 1e9
        // i=2: max=1e9, min=0 -> 1e9
        // i=3: max=1e9, min=0 -> 1e9
        assert_eq!(Solution::first_stable_index(nums.clone(), 999_999_999), -1);
        assert_eq!(Solution::first_stable_index(nums, 1_000_000_000), 0);
    }
}
