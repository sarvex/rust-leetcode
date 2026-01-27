impl Solution {
    /// Maximizes the product of integers that sum to n using DP.
    ///
    /// # Intuition
    /// For each integer i, try splitting off j and take the maximum of
    /// using j as-is `(i-j)*j` or using the optimal split `f[i-j]*j`.
    ///
    /// # Approach
    /// 1. Build a DP array where f[i] is the maximum product for integer i.
    /// 2. For each i, try all splits j from 1 to i-1.
    /// 3. f[i] = max(f[i-j]*j, (i-j)*j) for all j.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n)
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0usize; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..i {
                dp[i] = dp[i].max(dp[i - j] * j).max((i - j) * j);
            }
        }
        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_two() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn break_ten() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
