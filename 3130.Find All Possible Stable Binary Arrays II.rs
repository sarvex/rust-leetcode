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
    /// Stability means no run of identical bits exceeds `limit`, so the last run determines
    /// how the array can be extended.
    ///
    /// # Approach
    /// - Use DP on counts and last bit: `dp0[i][j]` ends with 0, `dp1[i][j]` ends with 1.
    /// - For each cell, extend the previous state and subtract the window that exceeds `limit`.
    /// - This yields O(1) transitions without extra prefix arrays.
    ///
    /// # Complexity
    /// - Time: O(zero * one)
    /// - Space: O(zero * one)
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zeros = zero as usize;
        let ones = one as usize;
        let limit = limit as usize;

        let mut dp = vec![vec![[0i64; 2]; ones + 1]; zeros + 1];

        for i in 1..=limit {
            if i <= zeros {
                dp[i][0][0] = 1;
            }
            if i <= ones {
                dp[0][i][1] = 1;
            }
        }

        for i in 1..=zeros {
            for j in 1..=ones {
                let mut zero_ending = mod_add(dp[i - 1][j][0], dp[i - 1][j][1]);
                if i > limit {
                    zero_ending = mod_sub(zero_ending, dp[i - limit - 1][j][1]);
                }

                let mut one_ending = mod_add(dp[i][j - 1][0], dp[i][j - 1][1]);
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
        assert_eq!(Solution::number_of_stable_arrays(2, 2, 1), 2);
    }

    #[test]
    fn test_large_limit_no_constraint() {
        assert_eq!(Solution::number_of_stable_arrays(2, 3, 5), 10);
    }
}
