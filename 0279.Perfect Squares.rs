impl Solution {
    /// Finds the minimum number of perfect squares summing to n using 1D DP.
    ///
    /// # Intuition
    /// This is an unbounded knapsack problem where perfect squares are items
    /// and n is the target weight. Use 1D DP for space efficiency.
    ///
    /// # Approach
    /// 1. Create a 1D DP array where dp[i] is the minimum squares summing to i.
    /// 2. For each value from 1 to n, try all perfect squares <= current value.
    /// 3. dp[i] = min(dp[i - j*j] + 1) for all valid j.
    ///
    /// # Complexity
    /// - Time: O(n * sqrt(n))
    /// - Space: O(n)
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                if dp[i - j * j] != i32::MAX {
                    dp[i] = dp[i].min(dp[i - j * j] + 1);
                }
                j += 1;
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_needs_three() {
        // 12 = 4 + 4 + 4 (three 2^2)
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn thirteen_needs_two() {
        // 13 = 9 + 4 (3^2 + 2^2)
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn perfect_square() {
        // 16 = 4^2 (one perfect square)
        assert_eq!(Solution::num_squares(16), 1);
    }

    #[test]
    fn one() {
        // 1 = 1^2
        assert_eq!(Solution::num_squares(1), 1);
    }

    #[test]
    fn two() {
        // 2 = 1 + 1
        assert_eq!(Solution::num_squares(2), 2);
    }

    #[test]
    fn three() {
        // 3 = 1 + 1 + 1
        assert_eq!(Solution::num_squares(3), 3);
    }

    #[test]
    fn four() {
        // 4 = 2^2
        assert_eq!(Solution::num_squares(4), 1);
    }

    #[test]
    fn five() {
        // 5 = 4 + 1
        assert_eq!(Solution::num_squares(5), 2);
    }

    #[test]
    fn larger_number() {
        // 100 = 10^2 or 64 + 36 = 8^2 + 6^2
        assert_eq!(Solution::num_squares(100), 1);
    }
}
