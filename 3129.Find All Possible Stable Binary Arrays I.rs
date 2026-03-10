const MOD: i64 = 1_000_000_007;

fn mod_add(left: i64, right: i64) -> i64 {
    let value = left + right;
    if value >= MOD {
        value - MOD
    } else {
        value
    }
}

fn mod_sub(left: i64, right: i64) -> i64 {
    let mut value = left - right;
    if value < 0 {
        value += MOD;
    }
    value
}

impl Solution {
    /// Counts stable binary arrays with bounded run lengths.
    ///
    /// # Intuition
    /// A stable array has no run of identical bits exceeding `limit`. By tracking
    /// the last bit placed, we can count valid extensions and use a sliding window
    /// to subtract invalid configurations.
    ///
    /// # Approach
    /// - Use DP on counts: `dp[i][j][k]` = arrays with `i` zeros, `j` ones,
    ///   ending with bit `k` (0 or 1)
    /// - Transition: extend by placing another 0 or 1, but subtract states where
    ///   the run would exceed `limit`
    /// - This yields O(1) transitions using the inclusion-exclusion principle
    ///
    /// # Complexity
    /// - Time: O(zero * one)
    /// - Space: O(zero * one)
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zeros = zero as usize;
        let ones = one as usize;
        let limit = limit as usize;

        // dp[i][j][k] = number of stable arrays with i zeros, j ones, ending with bit k
        let mut dp = vec![vec![[0i64; 2]; ones + 1]; zeros + 1];

        // Initialize base cases: runs of only zeros or only ones (within limit)
        for i in 1..=limit {
            if i <= zeros {
                dp[i][0][0] = 1;
            }
            if i <= ones {
                dp[0][i][1] = 1;
            }
        }

        // Fill DP table
        for i in 1..=zeros {
            for j in 1..=ones {
                // Extend array ending with 0: add a 0 to arrays ending with 0 or 1
                let mut zero_ending = mod_add(dp[i - 1][j][0], dp[i - 1][j][1]);
                // Subtract arrays where adding this 0 would create a run > limit
                if i > limit {
                    zero_ending = mod_sub(zero_ending, dp[i - limit - 1][j][1]);
                }

                // Extend array ending with 1: add a 1 to arrays ending with 0 or 1
                let mut one_ending = mod_add(dp[i][j - 1][0], dp[i][j - 1][1]);
                // Subtract arrays where adding this 1 would create a run > limit
                if j > limit {
                    one_ending = mod_sub(one_ending, dp[i][j - limit - 1][0]);
                }

                dp[i][j][0] = zero_ending;
                dp[i][j][1] = one_ending;
            }
        }

        mod_add(dp[zeros][ones][0], dp[zeros][ones][1]) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }

    #[test]
    fn test_limit_one_alternating() {
        // With limit=1, we must alternate: 0101 or 1010 pattern
        assert_eq!(Solution::number_of_stable_arrays(2, 2, 1), 2);
    }

    #[test]
    fn test_large_limit_no_constraint() {
        // If limit >= total length, all permutations are valid
        assert_eq!(Solution::number_of_stable_arrays(2, 3, 5), 10);
    }

    #[test]
    fn test_single_zero() {
        assert_eq!(Solution::number_of_stable_arrays(1, 3, 2), 4);
    }

    #[test]
    fn test_single_one() {
        assert_eq!(Solution::number_of_stable_arrays(3, 1, 2), 4);
    }

    #[test]
    fn test_limit_zero_impossible() {
        // limit = 0 means no subarray > 0 can have identical elements
        // This is impossible unless array is empty
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 0), 0);
    }
}
