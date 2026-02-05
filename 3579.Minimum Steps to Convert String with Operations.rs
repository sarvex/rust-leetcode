impl Solution {
    /// Minimum operations to convert string using segment DP with reverse and swap.
    ///
    /// # Intuition
    /// Each contiguous substring can be transformed independently using reverse,
    /// swap, and replace operations. We use dynamic programming to find the optimal
    /// partitioning of the string, computing the cost for each possible segment.
    ///
    /// # Approach
    /// 1. Precompute no-reverse costs incrementally: for each starting position j,
    ///    extend right while maintaining mismatch count and swap count in O(1) per step
    /// 2. Precompute reversed costs for each segment without allocation
    /// 3. DP: dp[i] = min over j of (dp[j] + min(cost_no_rev[j][i], 1 + cost_rev[j][i]))
    /// 4. Swap counting optimization: when adding mismatch (s,t), swaps increase by 1
    ///    only if count[t][s] > count[s][t] (greedy pairing)
    ///
    /// # Complexity
    /// - Time: O(n²) for no-reverse precomputation + O(n³) for reversed = O(n³)
    /// - Space: O(n²) for cost matrices
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let w1 = word1.into_bytes();
        let w2 = word2.into_bytes();
        let n = w1.len();

        // Precompute costs for all segments
        let mut cost_no_rev = vec![vec![0i32; n + 1]; n];
        let mut cost_rev = vec![vec![0i32; n + 1]; n];

        for j in 0..n {
            // No-reverse: incremental O(n) total for all segments starting at j
            let mut count = [[0i32; 26]; 26];
            let mut mismatches = 0i32;
            let mut swaps = 0i32;

            for i in (j + 1)..=n {
                let s = (w1[i - 1] - b'a') as usize;
                let t = (w2[i - 1] - b'a') as usize;

                if s != t {
                    mismatches += 1;
                    // Swap count increases only when adding creates a beneficial pair
                    if count[t][s] > count[s][t] {
                        swaps += 1;
                    }
                    count[s][t] += 1;
                }

                cost_no_rev[j][i] = mismatches - swaps;
            }

            // Reversed: compute directly for each segment, no allocation
            for i in (j + 1)..=n {
                let mut cnt = [[0i32; 26]; 26];
                let mut mismatch = 0i32;
                let len = i - j;

                for k in 0..len {
                    let s = (w1[i - 1 - k] - b'a') as usize;
                    let t = (w2[j + k] - b'a') as usize;
                    if s != t {
                        mismatch += 1;
                        cnt[s][t] += 1;
                    }
                }

                let swap_count: i32 = (0..26)
                    .flat_map(|a| ((a + 1)..26).map(move |b| cnt[a][b].min(cnt[b][a])))
                    .sum();

                cost_rev[j][i] = mismatch - swap_count;
            }
        }

        // DP with O(1) cost lookup per segment
        let dp = (1..=n).fold(
            {
                let mut dp = vec![i32::MAX; n + 1];
                dp[0] = 0;
                dp
            },
            |mut dp, i| {
                dp[i] = (0..i)
                    .map(|j| dp[j] + cost_no_rev[j][i].min(1 + cost_rev[j][i]))
                    .min()
                    .unwrap_or(i32::MAX);
                dp
            },
        );

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::min_operations("abcdf".to_string(), "dacbe".to_string()),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::min_operations("abceded".to_string(), "baecfef".to_string()),
            4
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::min_operations("abcdef".to_string(), "fedabc".to_string()),
            2
        );
    }

    #[test]
    fn test_identical_strings() {
        assert_eq!(
            Solution::min_operations("abc".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(
            Solution::min_operations("a".to_string(), "a".to_string()),
            0
        );
    }

    #[test]
    fn test_single_char_mismatch() {
        assert_eq!(
            Solution::min_operations("a".to_string(), "b".to_string()),
            1
        );
    }

    #[test]
    fn test_simple_swap() {
        assert_eq!(
            Solution::min_operations("ab".to_string(), "ba".to_string()),
            1
        );
    }

    #[test]
    fn test_simple_reverse() {
        assert_eq!(
            Solution::min_operations("abc".to_string(), "cba".to_string()),
            1
        );
    }

    #[test]
    fn test_all_different() {
        assert_eq!(
            Solution::min_operations("aaa".to_string(), "bbb".to_string()),
            3
        );
    }

    #[test]
    fn test_partial_match() {
        assert_eq!(
            Solution::min_operations("abcd".to_string(), "abce".to_string()),
            1
        );
    }
}
