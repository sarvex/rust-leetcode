use std::collections::HashMap;

impl Solution {
    /// Chunk-Based Prefix Sum Counting (Optimized)
    ///
    /// # Intuition
    /// For a sorted array, process runs of identical values. For each run,
    /// count subarrays ending at each position using prefix sum remainders.
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

        let mut result = 0i64;
        let mut sum = 0usize;

        let n = nums.len();
        let mut i = 0;

        while i < n {
            let x = (nums[i] as usize) % k;
            let start = i;
            while i < n && nums[i] == nums[start] {
                i += 1;
            }
            let cnt = i - start;

            let mut s = sum;
            for _ in 0..cnt {
                s = (s + x) % k;
                result += sums[s];
            }

            for _ in 0..cnt {
                sum = (sum + x) % k;
                sums[sum] += 1;
            }
        }

        result
    }

    fn solve_with_map(nums: &[i32], k: i32) -> i64 {
        let mut sums: HashMap<i32, i64> = HashMap::new();
        sums.insert(0, 1);

        let mut result = 0i64;
        let mut sum = 0i32;

        let n = nums.len();
        let mut i = 0;

        while i < n {
            let x = ((nums[i] % k) + k) % k;
            let start = i;
            while i < n && nums[i] == nums[start] {
                i += 1;
            }
            let cnt = i - start;

            let mut s = sum;
            for _ in 0..cnt {
                s = (s + x) % k;
                result += sums.get(&s).copied().unwrap_or(0);
            }

            for _ in 0..cnt {
                sum = (sum + x) % k;
                *sums.entry(sum).or_insert(0) += 1;
            }
        }

        result
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
