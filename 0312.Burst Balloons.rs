impl Solution {
    /// Maximizes coins from bursting balloons using interval DP.
    ///
    /// # Intuition
    /// Think of the last balloon burst in each interval. The coins earned
    /// depend on the boundary balloons (already burst) and the sub-intervals.
    ///
    /// # Approach
    /// 1. Pad the array with 1s at both ends.
    /// 2. Define dp[i][j] as the max coins obtainable from bursting all balloons
    ///    strictly between indices i and j.
    /// 3. For each interval, try every balloon k as the last to burst.
    /// 4. dp[i][j] = max(dp[i][k] + dp[k][j] + arr[i] * arr[k] * arr[j]).
    ///
    /// # Complexity
    /// - Time: O(n^3)
    /// - Space: O(n^2)
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut arr = vec![1; n + 2];
        for (i, &v) in nums.iter().enumerate() {
            arr[i + 1] = v;
        }
        let mut dp = vec![vec![0; n + 2]; n + 2];
        for len in 2..=n + 1 {
            for i in 0..=n + 1 - len {
                let j = i + len;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j] + arr[i] * arr[k] * arr[j]);
                }
            }
        }
        dp[0][n + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }

    #[test]
    fn single_balloon() {
        assert_eq!(Solution::max_coins(vec![5]), 5);
    }

    #[test]
    fn two_balloons() {
        // [3, 1] -> burst 1 first: 3*1*1 + 1*3*1 = 3 + 3 = 6
        // or burst 3 first: 1*3*1 + 1*1*1 = 3 + 1 = 4
        assert_eq!(Solution::max_coins(vec![3, 1]), 6);
    }

    #[test]
    fn all_ones() {
        // [1, 1, 1] -> always 1*1*1 = 3
        assert_eq!(Solution::max_coins(vec![1, 1, 1]), 3);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::max_coins(vec![]), 0);
    }
}
