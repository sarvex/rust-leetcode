impl Solution {
    /// Dynamic programming solution for maximizing cyclic partition score
    ///
    /// # Intuition
    /// For a cyclic array, we try all possible starting positions for the first partition.
    /// The key optimization is precomputing all ranges for O(1) lookup.
    ///
    /// # Approach
    /// 1. Precompute ranges for all cyclic subarrays in O(n²)
    /// 2. Try each position as the start of the first partition
    /// 3. dp[i][j] = max score partitioning first i elements into j partitions
    /// 4. Use O(1) range lookups from precomputed table
    ///
    /// # Complexity
    /// - Time complexity: O(n² × k) per starting position
    /// - Space complexity: O(n²) for range cache
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k.min(n as i32) as usize;

        if n == 1 {
            return 0;
        }

        // Precompute: range[i][len] = range of subarray starting at index i with length len+1
        let mut range = vec![vec![0i64; n]; n];
        for i in 0..n {
            let mut min_val = nums[i] as i64;
            let mut max_val = min_val;
            for len in 0..n {
                let val = nums[(i + len) % n] as i64;
                min_val = min_val.min(val);
                max_val = max_val.max(val);
                range[i][len] = max_val - min_val;
            }
        }

        // Try each starting position and find the maximum
        let mut best = 0i64;
        let mut dp = vec![vec![i64::MIN; k + 1]; n + 1];

        for start in 0..n {
            // Reset DP
            for row in dp.iter_mut() {
                row.fill(i64::MIN);
            }
            dp[0][0] = 0;

            for i in 1..=n {
                for j in 1..=k.min(i) {
                    for prev in (j - 1)..i {
                        if dp[prev][j - 1] != i64::MIN {
                            let r = range[(start + prev) % n][i - prev - 1];
                            dp[i][j] = dp[i][j].max(dp[prev][j - 1] + r);
                        }
                    }
                }
            }

            for j in 1..=k {
                if dp[n][j] != i64::MIN {
                    best = best.max(dp[n][j]);
                }
            }
        }

        best
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 3];
        let k = 2;
        assert_eq!(Solution::maximum_score(nums, k), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3, 3];
        let k = 1;
        assert_eq!(Solution::maximum_score(nums, k), 2);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3, 3];
        let k = 4;
        assert_eq!(Solution::maximum_score(nums, k), 3);
    }

    #[test]
    fn test_failed_case_1() {
        let nums = vec![1, 1000000000, 1000000000];
        let k = 3;
        assert_eq!(Solution::maximum_score(nums, k), 999999999);
    }

    #[test]
    fn test_failed_case_2() {
        let nums = vec![563861991,113020722,111783372,603350366,771729177,91633423,664375708,85118934];
        let k = 4;
        assert_eq!(Solution::maximum_score(nums, k), 2201760791);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![5];
        let k = 1;
        assert_eq!(Solution::maximum_score(nums, k), 0);
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 10];
        let k = 1;
        assert_eq!(Solution::maximum_score(nums, k), 9);
    }

    #[test]
    fn test_two_elements_two_partitions() {
        let nums = vec![1, 10];
        let k = 2;
        assert_eq!(Solution::maximum_score(nums, k), 9);
    }

    #[test]
    fn test_uniform_array() {
        let nums = vec![5, 5, 5, 5];
        let k = 2;
        assert_eq!(Solution::maximum_score(nums, k), 0);
    }

    #[test]
    fn test_increasing_array() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        let result = Solution::maximum_score(nums, k);
        assert!(result >= 4);
    }
}
