use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /// Binary search with greedy savings under suffix capacity.
    ///
    /// # Intuition
    /// Without any "set" operations, we must spend `sum(nums)` decrements plus
    /// `n` marks. Using a set for index `i` (with `nums[i] > 0`) replaces
    /// `nums[i]` decrements by one set operation, saving `nums[i] - 1` seconds.
    /// The remaining constraint is that each set still needs a *later* second
    /// to mark the index, so in any suffix of length `L`, we can choose at most
    /// `floor(L / 2)` set operations.
    ///
    /// # Approach
    /// Binary search the earliest feasible second `t`. For a fixed `t`:
    /// - Only the *earliest* occurrence of an index can be its best set time:
    ///   moving a set earlier can only relax suffix constraints.
    /// - Each earliest occurrence with `nums[i] > 1` contributes a saving of
    ///   `nums[i] - 1` if chosen.
    /// - Scan from time `t` down to `1`. Keep a min-heap of chosen savings.
    ///   After processing a suffix of length `len`, keep at most `len / 2`
    ///   candidates by dropping the smallest saving. This yields the maximum
    ///   total savings possible under all suffix constraints.
    /// - `t` is feasible if `sum(nums) + n - max_saving <= t`.
    ///
    /// # Complexity
    /// - Time: O(m log^2 m)
    /// - Space: O(m + n)
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = change_indices.len();
        let nums: Vec<i64> = nums.into_iter().map(|value| value as i64).collect();
        let change: Vec<usize> = change_indices
            .into_iter()
            .map(|value| value as usize)
            .collect();
        let total_sum: i64 = nums.iter().sum();

        if !Self::can_mark_by(m, &nums, &change, total_sum) {
            return -1;
        }

        let mut left = 1usize;
        let mut right = m;
        let mut answer = m;
        while left <= right {
            let mid = (left + right) / 2;
            if Self::can_mark_by(mid, &nums, &change, total_sum) {
                answer = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        answer as i32
    }

    fn can_mark_by(t: usize, nums: &[i64], change: &[usize], total_sum: i64) -> bool {
        let max_saving = Self::max_saving(t, nums, change);
        total_sum + nums.len() as i64 - max_saving <= t as i64
    }

    fn max_saving(t: usize, nums: &[i64], change: &[usize]) -> i64 {
        let n = nums.len();
        let mut first = vec![0usize; n];
        for (second, &idx) in change.iter().take(t).enumerate() {
            let index = idx - 1;
            if first[index] == 0 {
                first[index] = second + 1;
            }
        }

        let mut saving_at = vec![0i64; t + 1];
        for (index, &value) in nums.iter().enumerate() {
            let second = first[index];
            if second != 0 && second < t {
                let saving = value - 1;
                if saving > 0 {
                    saving_at[second] = saving;
                }
            }
        }

        let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut total_saving = 0i64;
        let mut chosen = 0usize;
        let mut suffix_len = 0usize;

        for second in (1..=t).rev() {
            suffix_len += 1;
            if second < t {
                let saving = saving_at[second];
                if saving > 0 {
                    heap.push(Reverse(saving));
                    total_saving += saving;
                    chosen += 1;
                }
            }

            let limit = suffix_len / 2;
            if chosen > limit {
                if let Some(Reverse(removed)) = heap.pop() {
                    total_saving -= removed;
                    chosen -= 1;
                }
            }
        }

        total_saving
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]),
            6
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(
                vec![0, 0, 1, 2],
                vec![1, 2, 1, 2, 1, 2, 1, 2]
            ),
            7
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }

    #[test]
    fn test_all_zero_marks_only() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![0, 0], vec![1, 2]),
            2
        );
    }

    #[test]
    fn test_late_sets_blocked_by_suffix() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![4, 4, 0], vec![3, 3, 3, 1, 2, 3]),
            -1
        );
    }
}
