impl Solution {
    /// DP with precomputed palindrome conversion costs.
    ///
    /// # Intuition
    /// Precompute the cost to convert any substring `s[i..=j]` into a palindrome
    /// by counting mismatched symmetric pairs. Then use DP where `f[i][j]` is the
    /// minimum total changes to partition the first `i` characters into `j`
    /// palindromic substrings.
    ///
    /// # Approach
    /// 1. Build cost matrix `g[i][j]` = number of changes to make `s[i..=j]` a palindrome
    /// 2. DP: `f[i][1] = g[0][i-1]`; for `j > 1`, try all split points
    /// 3. Return `f[n][k]`
    ///
    /// # Complexity
    /// - Time: O(n² · k) for the DP, O(n²) for cost precomputation
    /// - Space: O(n² + n·k)
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        let mut cost = vec![vec![0i32; n]; n];
        for i in (0..n).rev() {
            for j in i + 1..n {
                cost[i][j] = if bytes[i] != bytes[j] { 1 } else { 0 }
                    + if i + 1 < j { cost[i + 1][j - 1] } else { 0 };
            }
        }

        let mut dp = vec![vec![i32::MAX; k + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=i.min(k) {
                if j == 1 {
                    dp[i][j] = cost[0][i - 1];
                } else {
                    dp[i][j] = (j - 1..i)
                        .filter(|&h| dp[h][j - 1] != i32::MAX)
                        .map(|h| dp[h][j - 1] + cost[h][i - 1])
                        .min()
                        .unwrap_or(i32::MAX);
                }
            }
        }

        dp[n][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_partitions() {
        assert_eq!(Solution::palindrome_partition("abc".to_string(), 2), 1);
    }

    #[test]
    fn already_palindromes() {
        assert_eq!(Solution::palindrome_partition("aabbc".to_string(), 3), 0);
    }

    #[test]
    fn single_partition() {
        assert_eq!(Solution::palindrome_partition("leetcode".to_string(), 8), 0);
    }
}
