use std::collections::HashMap;

impl Solution {
    /// Chunk-based prefix sum counting for divisible subarrays in sorted arrays.
    ///
    /// # Intuition
    /// For a sorted array, process runs of identical values. For each run,
    /// count subarrays ending at each position using prefix sum remainders.
    ///
    /// # Approach
    /// 1. Group consecutive identical elements into chunks
    /// 2. Track prefix sum remainders mod k in a frequency table
    /// 3. For each chunk, accumulate matching remainder counts before updating
    /// 4. Use Vec for small k, HashMap for large k to optimize space
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(min(n, k))
    pub fn num_good_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();

        // Use Vec for small k, HashMap for large k
        if k <= 100_000 {
            Self::solve_with_vec(&nums, k as usize)
        } else {
            Self::solve_with_map(&nums, k)
        }
    }

    fn solve_with_vec(nums: &[i32], k: usize) -> i64 {
        let mut sums = vec![0i64; k];
        sums[0] = 1;

        // Group consecutive identical elements into chunks: (value % k, count)
        let chunks: Vec<(usize, usize)> = {
            let mut result = Vec::new();
            let mut i = 0;
            while i < nums.len() {
                let x = (nums[i] as usize) % k;
                let start = i;
                while i < nums.len() && nums[i] == nums[start] {
                    i += 1;
                }
                result.push((x, i - start));
            }
            result
        };

        chunks
            .iter()
            .fold((0i64, 0usize), |(result, sum), &(x, cnt)| {
                // Count matches: sum up sums[(sum + i*x) % k] for i in 1..=cnt
                let match_count: i64 = (1..=cnt).map(|i| sums[(sum + i * x) % k]).sum();

                // Update sums and compute new sum
                let new_sum = (1..=cnt).fold(sum, |s, _| {
                    let next_s = (s + x) % k;
                    sums[next_s] += 1;
                    next_s
                });

                (result + match_count, new_sum)
            })
            .0
    }

    fn solve_with_map(nums: &[i32], k: i32) -> i64 {
        let mut sums: HashMap<i32, i64> = HashMap::new();
        sums.insert(0, 1);

        // Group consecutive identical elements into chunks: (value % k, count)
        let chunks: Vec<(i32, usize)> = {
            let mut result = Vec::new();
            let mut i = 0;
            while i < nums.len() {
                let x = ((nums[i] % k) + k) % k;
                let start = i;
                while i < nums.len() && nums[i] == nums[start] {
                    i += 1;
                }
                result.push((x, i - start));
            }
            result
        };

        chunks
            .iter()
            .fold((0i64, 0i32), |(result, sum), &(x, cnt)| {
                // Count matches
                let match_count: i64 = (1..=cnt)
                    .map(|i| {
                        let target = (sum + (i as i32) * x) % k;
                        sums.get(&target).copied().unwrap_or(0)
                    })
                    .sum();

                // Update sums and compute new sum
                let new_sum = (1..=cnt).fold(sum, |s, _| {
                    let next_s = (s + x) % k;
                    *sums.entry(next_s).or_insert(0) += 1;
                    next_s
                });

                (result + match_count, new_sum)
            })
            .0
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
