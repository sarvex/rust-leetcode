impl Solution {
    /// Counting stable subarrays using run starts and prefix sums
    ///
    /// # Intuition
    /// A stable subarray is non-decreasing. For each position, track where its
    /// non-decreasing run starts. Use prefix sums to compute excess contributions
    /// from positions whose runs start after the query's left bound.
    ///
    /// # Approach
    /// 1. Compute run_start[i] = earliest index of the run ending at i
    /// 2. Build prefix sums of run_start values
    /// 3. Precompute first_greater[l] = first index where run_start > l
    /// 4. For each query, subtract excess from baseline triangular count
    ///
    /// # Complexity
    /// - Time: O(n + q)
    /// - Space: O(n)
    pub fn count_stable_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let num_elements = nums.len();

        let run_start: Vec<usize> = nums.windows(2).fold(vec![0usize], |mut acc, w| {
            let prev = *acc.last().unwrap();
            acc.push(if w[0] <= w[1] { prev } else { acc.len() });
            acc
        });

        let prefix_sum: Vec<i64> = std::iter::once(0i64)
            .chain(run_start.iter().scan(0i64, |sum, &r| {
                *sum += r as i64;
                Some(*sum)
            }))
            .collect();

        let mut first_greater = vec![num_elements; num_elements];
        let mut pointer = 0usize;
        for left_bound in 0..num_elements {
            while pointer < num_elements && run_start[pointer] <= left_bound {
                pointer += 1;
            }
            first_greater[left_bound] = pointer;
        }

        queries
            .iter()
            .map(|query| {
                let (left_bound, right_bound) = (query[0] as usize, query[1] as usize);
                let query_length = (right_bound - left_bound + 1) as i64;
                let baseline_count = query_length * (query_length + 1) / 2;

                let threshold = first_greater[left_bound];
                if threshold <= right_bound {
                    let affected_count = (right_bound - threshold + 1) as i64;
                    let sum_run_starts = prefix_sum[right_bound + 1] - prefix_sum[threshold];
                    let total_excess = sum_run_starts - affected_count * (left_bound as i64);
                    baseline_count - total_excess
                } else {
                    baseline_count
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 1, 2];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        assert_eq!(
            Solution::count_stable_subarrays(nums, queries),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 2];
        let queries = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::count_stable_subarrays(nums, queries), vec![3, 1]);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::count_stable_subarrays(nums, queries), vec![1]);
    }

    #[test]
    fn test_fully_sorted() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![0, 3]];
        assert_eq!(Solution::count_stable_subarrays(nums, queries), vec![10]);
    }

    #[test]
    fn test_fully_descending() {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![0, 3]];
        assert_eq!(Solution::count_stable_subarrays(nums, queries), vec![4]);
    }
}
