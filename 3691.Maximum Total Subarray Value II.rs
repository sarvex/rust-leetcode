use std::collections::BinaryHeap;

impl Solution {
    /// Finds the maximum total value from selecting k distinct subarrays.
    ///
    /// # Intuition
    /// Larger subarrays have values >= their sub-intervals. By initializing
    /// with all subarrays ending at n-1 and only shrinking right, each
    /// subarray is visited exactly once without needing a visited set.
    ///
    /// # Approach
    /// 1. Build flattened sparse table for O(1) range min/max queries
    /// 2. Initialize heap with all subarrays [l, n-1] for each l
    /// 3. Pop maximum, add child [l, r-1] (shrink right only)
    ///
    /// # Complexity
    /// - Time: O(n log n + k log n)
    /// - Space: O(n log n)
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let array_length = nums.len();
        if array_length == 0 || k <= 0 {
            return 0;
        }

        let num_to_select = (k as usize).min(array_length * (array_length + 1) / 2);

        // Precompute floor(log2) table
        let mut floor_log2 = vec![0usize; array_length + 1];
        (2..=array_length).for_each(|i| floor_log2[i] = floor_log2[i >> 1] + 1);

        let num_levels = floor_log2[array_length] + 1;

        // Flattened sparse tables: index = level * array_length + position
        // Single contiguous allocation for better cache locality
        let mut sparse_max = vec![i32::MIN; num_levels * array_length];
        let mut sparse_min = vec![i32::MAX; num_levels * array_length];

        // Level 0: copy original array values
        sparse_max[..array_length].copy_from_slice(&nums);
        sparse_min[..array_length].copy_from_slice(&nums);

        // Build higher levels by combining adjacent intervals
        (1..num_levels).for_each(|level| {
            let half_span = 1 << (level - 1);
            let prev_base = (level - 1) * array_length;
            let curr_base = level * array_length;
            (0..=array_length.saturating_sub(1 << level)).for_each(|start| {
                sparse_max[curr_base + start] =
                    sparse_max[prev_base + start].max(sparse_max[prev_base + start + half_span]);
                sparse_min[curr_base + start] =
                    sparse_min[prev_base + start].min(sparse_min[prev_base + start + half_span]);
            });
        });

        // Query subarray value using sparse table lookups
        let subarray_value = |left: usize, right: usize| -> i64 {
            let length = right - left + 1;
            let level = floor_log2[length];
            let base = level * array_length;
            let span = 1 << level;
            let maximum = sparse_max[base + left].max(sparse_max[base + right + 1 - span]);
            let minimum = sparse_min[base + left].min(sparse_min[base + right + 1 - span]);
            (maximum - minimum) as i64
        };

        // Max-heap using tuples: (value, left, right)
        // Tuples compare lexicographically - perfect for max-heap ordering
        let mut priority_queue: BinaryHeap<(i64, usize, usize)> =
            BinaryHeap::with_capacity(array_length);

        // Initialize with all subarrays ending at last index
        let last_index = array_length - 1;
        (0..array_length).for_each(|left| {
            priority_queue.push((subarray_value(left, last_index), left, last_index));
        });

        let mut accumulated_sum = 0i64;

        (0..num_to_select).for_each(|_| {
            if let Some((value, left, right)) = priority_queue.pop() {
                accumulated_sum += value;
                // Add child subarray by shrinking from right
                if right > left {
                    let new_right = right - 1;
                    priority_queue.push((subarray_value(left, new_right), left, new_right));
                }
            }
        });

        accumulated_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_basic_array() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
    }

    #[test]
    fn example_overlapping_optimal() {
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }

    #[test]
    fn single_element_array() {
        assert_eq!(Solution::max_total_value(vec![5], 1), 0);
    }

    #[test]
    fn all_identical_elements() {
        assert_eq!(Solution::max_total_value(vec![3, 3, 3], 3), 0);
    }

    #[test]
    fn two_element_array() {
        assert_eq!(Solution::max_total_value(vec![1, 5], 2), 4);
    }

    #[test]
    fn select_all_subarrays() {
        // Subarrays of [1,2,3]: [1],[2],[3],[1,2],[2,3],[1,2,3]
        // Values: 0, 0, 0, 1, 1, 2 → sum = 4
        assert_eq!(Solution::max_total_value(vec![1, 2, 3], 6), 4);
    }
}
