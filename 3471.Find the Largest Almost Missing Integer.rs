impl Solution {
    /// Positional analysis of size-`k` subarray coverage with a direct count table.
    ///
    /// # Intuition
    /// An element at index `i` is covered by every size-`k` window whose start
    /// lies in `[max(0, i - k + 1), min(i, n - k)]`. The size of that range
    /// dictates how many subarrays a position belongs to:
    /// - `k == n`: a single window covers everything, so every value present
    ///   appears in exactly one subarray — the answer is the maximum element.
    /// - `k == 1`: each index is its own window, so a value appears in exactly
    ///   one subarray precisely when it occurs once in the whole array,
    ///   independent of position.
    /// - `1 < k < n`: the range collapses to a single window only at the two
    ///   extreme positions, index `0` (only the first window) and index `n - 1`
    ///   (only the last window); every interior index is spanned by at least two
    ///   windows. So a value qualifies only when it occurs once *and* sits at an
    ///   end.
    ///
    /// # Approach
    /// Because values are bounded by `0 <= nums[i] <= 50`, tally counts in a
    /// fixed 51-entry array indexed directly by value — this avoids all hashing
    /// overhead. Then branch on `k` per the cases above, selecting the largest
    /// qualifying value or `-1` when none exists.
    ///
    /// # Complexity
    /// - Time: O(n + V) — one pass to tally counts plus a bounded scan over the
    ///   value domain `V = 51`
    /// - Space: O(V) — constant 51-entry count table
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        const MAX_VAL: usize = 50;

        let n = nums.len();
        let k = k as usize;

        if k == n {
            return nums.iter().copied().max().unwrap_or(-1);
        }

        let mut count = [0u8; MAX_VAL + 1];
        for &num in &nums {
            count[num as usize] = count[num as usize].saturating_add(1);
        }

        if k == 1 {
            return (0..=MAX_VAL)
                .rev()
                .find(|&val| count[val] == 1)
                .map_or(-1, |val| val as i32);
        }

        let unique_at = |idx: usize| -> i32 {
            let val = nums[idx];
            if count[val as usize] == 1 {
                val
            } else {
                -1
            }
        };

        unique_at(0).max(unique_at(n - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
    }

    #[test]
    fn k_equals_length() {
        // Single window covers all; largest distinct value wins.
        assert_eq!(Solution::largest_integer(vec![5, 1, 4, 4], 4), 5);
    }

    #[test]
    fn single_element_window() {
        // k = 1: each index is its own window, so a value qualifies whenever it
        // occurs exactly once — position is irrelevant.
        assert_eq!(Solution::largest_integer(vec![1, 2, 3], 1), 3);
        assert_eq!(Solution::largest_integer(vec![3, 1, 7, 10, 0], 1), 10);
        assert_eq!(Solution::largest_integer(vec![5, 5, 5], 1), -1);
    }

    #[test]
    fn both_ends_repeat_elsewhere() {
        // Endpoints reappear in the interior, so neither is almost missing.
        assert_eq!(Solution::largest_integer(vec![2, 3, 2, 3], 2), -1);
    }

    #[test]
    fn only_start_qualifies() {
        assert_eq!(Solution::largest_integer(vec![8, 1, 1, 8], 2), -1);
        assert_eq!(Solution::largest_integer(vec![9, 1, 2, 1], 2), 9);
    }

    #[test]
    fn single_element_array() {
        assert_eq!(Solution::largest_integer(vec![42], 1), 42);
    }
}
