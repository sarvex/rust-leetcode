struct Solution;

impl Solution {
    /// Dynamic programming solution for maximizing cyclic partition score
    ///
    /// # Intuition
    /// Handle cyclic partitioning by separating into two cases: linear partition
    /// (first/last in different partitions) and wrap-around (first/last in same partition).
    ///
    /// # Approach
    /// 1. Precompute ranges for all linear subarrays in O(n²)
    /// 2. Use segment-based DP with efficient memory reuse
    /// 3. Track best scores incrementally to avoid redundant computation
    /// 4. Case 2: Use precomputed interval best scores with O(1) lookup
    ///
    /// # Complexity
    /// - Time complexity: O(n² × k)
    /// - Space complexity: O(n² + nk) for range cache and DP
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k.min(n as i32) as usize;

        if n == 1 {
            return 0;
        }

        // Precompute: range_val[i][j] = max - min of nums[i..=j]
        let mut range_val = vec![vec![0i64; n]; n];
        for i in 0..n {
            let mut min_val = nums[i] as i64;
            let mut max_val = min_val;
            for j in i..n {
                let val = nums[j] as i64;
                min_val = min_val.min(val);
                max_val = max_val.max(val);
                range_val[i][j] = max_val - min_val;
            }
        }

        // best_km1[l][r] = max score for [l,r] with at most k-1 partitions
        let mut best_km1 = vec![vec![0i64; n]; n];

        // For Case 1: Linear DP for [0, n-1]
        // dp[i][p] = max score for nums[0..i] with exactly p partitions
        let mut dp_linear = vec![vec![i64::MIN; k + 1]; n + 1];
        dp_linear[0][0] = 0;

        for i in 1..=n {
            for p in 1..=k.min(i) {
                for m in (p - 1)..i {
                    if dp_linear[m][p - 1] != i64::MIN {
                        let score = dp_linear[m][p - 1] + range_val[m][i - 1];
                        if score > dp_linear[i][p] {
                            dp_linear[i][p] = score;
                        }
                    }
                }
            }
        }

        let mut best = 0i64;
        for p in 1..=k {
            if dp_linear[n][p] > best {
                best = dp_linear[n][p];
            }
        }

        // For Case 2: Compute best_km1 for all intervals [l, r] where 1 <= l <= n-2
        // These are the possible middle segments in wrap-around configurations
        if k >= 2 {
            let km1 = k - 1;

            for l in 1..n - 1 {
                let max_len = n - 1 - l; // Maximum right endpoint is n-2
                if max_len == 0 {
                    continue;
                }

                let max_p = km1.min(max_len + 1);
                if max_p == 0 {
                    continue;
                }

                // dp[r_offset][p] = max score for [l, l+r_offset] with p partitions
                let mut dp = vec![vec![i64::MIN; max_p + 1]; max_len + 1];
                dp[0][1] = 0;
                best_km1[l][l] = 0;

                for r_offset in 1..=max_len {
                    let r = l + r_offset;
                    let len = r_offset + 1;
                    let curr_max_p = max_p.min(len);

                    dp[r_offset][1] = range_val[l][r];

                    for p in 2..=curr_max_p {
                        for m_offset in (p - 2)..r_offset {
                            if dp[m_offset][p - 1] != i64::MIN {
                                let score = dp[m_offset][p - 1] + range_val[l + m_offset + 1][r];
                                if score > dp[r_offset][p] {
                                    dp[r_offset][p] = score;
                                }
                            }
                        }
                    }

                    let mut best_score = 0i64;
                    for p in 1..=curr_max_p {
                        if dp[r_offset][p] > best_score {
                            best_score = dp[r_offset][p];
                        }
                    }
                    best_km1[l][r] = best_score;
                }
            }

            // Precompute prefix/suffix min/max for wrap-around range computation
            let mut prefix_min = vec![0i64; n];
            let mut prefix_max = vec![0i64; n];
            prefix_min[0] = nums[0] as i64;
            prefix_max[0] = nums[0] as i64;
            for i in 1..n {
                prefix_min[i] = prefix_min[i - 1].min(nums[i] as i64);
                prefix_max[i] = prefix_max[i - 1].max(nums[i] as i64);
            }

            let mut suffix_min = vec![0i64; n];
            let mut suffix_max = vec![0i64; n];
            suffix_min[n - 1] = nums[n - 1] as i64;
            suffix_max[n - 1] = nums[n - 1] as i64;
            for i in (0..n - 1).rev() {
                suffix_min[i] = suffix_min[i + 1].min(nums[i] as i64);
                suffix_max[i] = suffix_max[i + 1].max(nums[i] as i64);
            }

            // Try all wrap-around configurations
            for wrap_end in 0..n - 1 {
                let p_min = prefix_min[wrap_end];
                let p_max = prefix_max[wrap_end];

                for wrap_start in wrap_end + 2..n {
                    let wrap_min = p_min.min(suffix_min[wrap_start]);
                    let wrap_max = p_max.max(suffix_max[wrap_start]);
                    let wrap_range = wrap_max - wrap_min;

                    let mid_l = wrap_end + 1;
                    let mid_r = wrap_start - 1;

                    let score = if mid_l <= mid_r {
                        wrap_range + best_km1[mid_l][mid_r]
                    } else {
                        wrap_range
                    };

                    if score > best {
                        best = score;
                    }
                }
            }
        }

        best
    }
}

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
        let nums = vec![
            563861991, 113020722, 111783372, 603350366, 771729177, 91633423, 664375708, 85118934,
        ];
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

    #[test]
    fn test_large_case() {
        let nums = vec![
            162, 79, 100, 157, 111, 134, 17, 108, 121, 191, 78, 64, 5, 107, 76, 156, 115, 147, 152,
            124, 99, 176, 85, 187, 90, 155, 33, 89, 79, 148, 118, 37, 133, 53, 79, 22, 118, 36, 65,
            30, 43, 93, 132, 181, 123, 149, 11, 142, 154, 48, 174, 184, 108, 163, 52, 69, 3, 162,
            152, 90, 1, 170, 164, 82, 85, 26, 165, 50, 24, 140, 98, 11, 100, 183, 148, 74, 63, 33,
            143, 26, 89, 143, 54, 177, 49, 168, 193, 191, 134, 106, 166, 42, 112, 193, 163, 5, 71,
            65, 22, 1, 116, 76, 71, 200, 94, 11, 108, 196, 9, 159, 11, 51, 156, 1, 38, 23, 52, 131,
            165, 102, 105, 179, 81, 28, 26, 165, 62, 81, 22, 161, 46, 46, 137, 61, 181, 125, 135,
            157, 155, 24,
        ];
        let k = 95;
        let result = Solution::maximum_score(nums, k);
        assert_eq!(result, 5848);
    }
}
