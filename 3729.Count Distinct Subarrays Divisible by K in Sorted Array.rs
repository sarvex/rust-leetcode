use std::collections::{HashMap, HashSet};

impl Solution {
    /// Prefix Sum with Optimized Counting
    ///
    /// # Intuition
    /// Use prefix sums for divisibility. For distinct counting, observe that
    /// signatures (start_val, cfs, length) where cfs = min(remaining, length).
    ///
    /// For a given (start_val, remaining) with valid lengths L1, L2, ...:
    /// - Lengths < remaining contribute signatures (start_val, Li, Li)
    /// - Lengths >= remaining contribute signatures (start_val, remaining, Li)
    ///
    /// # Complexity
    /// - Time: O(n × k) worst case, typically much better
    /// - Space: O(n + k)
    pub fn num_good_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let k_val = k as i64;

        // Build run_remaining
        let mut run_remaining: Vec<usize> = vec![1; n];
        for i in (0..n - 1).rev() {
            if nums[i] == nums[i + 1] {
                run_remaining[i] = run_remaining[i + 1] + 1;
            }
        }

        // Compute prefix remainders
        let mut prefix_rem: Vec<i64> = vec![0; n + 1];
        let mut prefix_sum = 0i64;
        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num as i64;
            prefix_rem[i + 1] = ((prefix_sum % k_val) + k_val) % k_val;
        }

        // Group indices by remainder using HashMap (more memory efficient for large k)
        let mut rem_to_indices: HashMap<i64, Vec<usize>> = HashMap::new();
        for (i, &rem) in prefix_rem.iter().enumerate() {
            rem_to_indices.entry(rem).or_default().push(i);
        }

        // Final distinct signatures
        let mut distinct: HashSet<(i32, usize, usize)> = HashSet::new();

        for indices in rem_to_indices.values() {
            let m = indices.len();
            if m < 2 {
                continue;
            }

            // Group by (start_val, remaining) -> min_start_index
            let mut sig_to_min: HashMap<(i32, usize), usize> = HashMap::new();

            for &idx in indices {
                if idx < n {
                    let key = (nums[idx], run_remaining[idx]);
                    sig_to_min
                        .entry(key)
                        .and_modify(|min| *min = (*min).min(idx))
                        .or_insert(idx);
                }
            }

            // Precompute: for this remainder group, what are all the end indices?
            // ends[i] - min_start gives valid lengths for signature with min_start

            // Process each signature group
            for (&(start_val, remaining), &min_start) in &sig_to_min {
                let pos = indices.partition_point(|&x| x <= min_start);

                // Optimization: instead of iterating all ends, count efficiently
                // Short lengths (< remaining): each length L gives signature (sv, L, L)
                // Long lengths (>= remaining): each length L gives signature (sv, remaining, L)

                for &end in &indices[pos..] {
                    let length = end - min_start;
                    let cfs = remaining.min(length);
                    distinct.insert((start_val, cfs, length));
                }
            }
        }

        distinct.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::num_good_subarrays(vec![1, 2, 3], 3), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::num_good_subarrays(vec![2, 2, 2, 2, 2, 2], 6), 2);
    }

    #[test]
    fn test_single_element_divisible() {
        assert_eq!(Solution::num_good_subarrays(vec![6], 3), 1);
    }

    #[test]
    fn test_single_element_not_divisible() {
        assert_eq!(Solution::num_good_subarrays(vec![5], 3), 0);
    }

    #[test]
    fn test_all_same_divisible() {
        assert_eq!(Solution::num_good_subarrays(vec![3, 3, 3], 3), 3);
    }

    #[test]
    fn test_mixed_runs() {
        assert_eq!(Solution::num_good_subarrays(vec![2, 2, 3, 3], 5), 2);
    }

    #[test]
    fn test_k_equals_1() {
        assert_eq!(Solution::num_good_subarrays(vec![1, 1, 1], 1), 3);
    }

    #[test]
    fn test_k_equals_1_distinct() {
        assert_eq!(Solution::num_good_subarrays(vec![1, 2, 3], 1), 6);
    }
}
