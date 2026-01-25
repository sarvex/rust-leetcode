use std::collections::HashMap;

impl Solution {
    /// Finds majority element meeting threshold frequency in subarrays.
    ///
    /// # Intuition
    /// Use Mo's algorithm with lazy max tracking. Maintain sliding window over
    /// sorted queries, updating max element incrementally during expansions.
    /// Only recompute max via full scan when invalidated by contractions.
    ///
    /// # Approach
    /// 1. Coordinate compress values for array-based counting
    /// 2. Sort queries using Mo's ordering (block, right endpoint, reverse left)
    /// 3. Expand window: update max incrementally if still valid
    /// 4. Contract window: invalidate max if affected element removed
    /// 5. Recompute max lazily only when needed for query answer
    ///
    /// # Complexity
    /// - Time: O((n + q) * sqrt(n)) amortized for Mo's algorithm
    /// - Space: O(n) for coordinate compression and frequency counts
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();

        let mut compressed = vec![0usize; n];
        let mut val_to_idx = HashMap::new();
        let mut idx_to_val = Vec::with_capacity(n / 4);

        for (i, &val) in nums.iter().enumerate() {
            compressed[i] = *val_to_idx.entry(val).or_insert_with(|| {
                let idx = idx_to_val.len();
                idx_to_val.push(val);
                idx
            });
        }

        let num_distinct = idx_to_val.len();
        let block_size = 1 + n.isqrt() / 2;

        let mut sorted_queries: Vec<_> = queries
            .iter()
            .enumerate()
            .map(|(i, q)| (q[0] as usize, q[1] as usize + 1, q[2], i))
            .collect();

        sorted_queries.sort_unstable_by_key(|q| (q.0 / block_size, q.1, n - q.0));

        let mut freq = vec![0i32; num_distinct];
        let mut results = vec![-1i32; queries.len()];
        let mut window_left = 0usize;
        let mut window_right = 0usize;
        let mut max_idx = num_distinct;
        let mut max_count = -1i32;

        for (target_left, target_right, threshold, query_idx) in sorted_queries {
            // Contract left
            while window_left < target_left {
                let idx = compressed[window_left];
                freq[idx] -= 1;
                if max_idx == idx {
                    max_idx = num_distinct;
                    max_count = -1;
                }
                window_left += 1;
            }

            // Contract right
            while window_right > target_right {
                window_right -= 1;
                let idx = compressed[window_right];
                freq[idx] -= 1;
                if max_idx == idx {
                    max_idx = num_distinct;
                    max_count = -1;
                }
            }

            // Expand left
            while window_left > target_left {
                window_left -= 1;
                let idx = compressed[window_left];
                freq[idx] += 1;
                if max_idx != num_distinct
                    && (freq[idx] > max_count
                        || (freq[idx] == max_count && idx_to_val[idx] < idx_to_val[max_idx]))
                {
                    max_count = freq[idx];
                    max_idx = idx;
                }
            }

            // Expand right
            while window_right < target_right {
                let idx = compressed[window_right];
                freq[idx] += 1;
                if max_idx != num_distinct
                    && (freq[idx] > max_count
                        || (freq[idx] == max_count && idx_to_val[idx] < idx_to_val[max_idx]))
                {
                    max_count = freq[idx];
                    max_idx = idx;
                }
                window_right += 1;
            }

            // Lazy recomputation when max invalidated
            if max_idx == num_distinct {
                for i in 0..num_distinct {
                    if freq[i] > max_count
                        || (freq[i] == max_count
                            && max_idx < num_distinct
                            && idx_to_val[i] < idx_to_val[max_idx])
                    {
                        max_idx = i;
                        max_count = freq[i];
                    }
                }
            }

            if threshold <= max_count {
                results[query_idx] = idx_to_val[max_idx];
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 1, 2, 2, 1, 1];
        let queries = vec![vec![0, 5, 4], vec![0, 3, 3], vec![2, 3, 2]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![1, -1, 2]);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 3, 2, 3, 2, 3];
        let queries = vec![vec![0, 6, 4], vec![1, 5, 2], vec![2, 4, 1], vec![3, 3, 1]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![3, 2, 3, 2]);
    }

    #[test]
    fn single_element_query() {
        let nums = vec![5];
        let queries = vec![vec![0, 0, 1]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![5]);
    }

    #[test]
    fn tie_breaking_smallest_value() {
        let nums = vec![1, 2, 1, 2];
        let queries = vec![vec![0, 3, 2]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![1]);
    }

    #[test]
    fn no_element_meets_threshold() {
        let nums = vec![1, 2, 3, 4, 5];
        let queries = vec![vec![0, 4, 2]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![-1]);
    }

    #[test]
    fn higher_frequency_wins() {
        let nums = vec![1, 1, 1, 2, 2];
        let queries = vec![vec![0, 4, 2]];
        assert_eq!(Solution::subarray_majority(nums, queries), vec![1]);
    }

    #[test]
    fn all_same_elements() {
        let nums = vec![8, 8];
        let queries = vec![
            vec![0, 0, 1],
            vec![0, 1, 2],
            vec![1, 1, 1],
            vec![0, 1, 2],
            vec![1, 1, 1],
        ];
        assert_eq!(
            Solution::subarray_majority(nums, queries),
            vec![8, 8, 8, 8, 8]
        );
    }
}
