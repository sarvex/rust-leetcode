pub struct Solution;

impl Solution {
    /// 2D dynamic programming for counting distinct subsequences.
    ///
    /// # Intuition
    /// `dp[i][j]` represents the number of ways to form `t[0..j]` from
    /// `s[0..i]`. If `s[i-1] == t[j-1]`, we can either use or skip this
    /// character; otherwise we must skip it.
    ///
    /// # Approach
    /// Initialize `dp[i][0] = 1` for all i (empty t matches any prefix).
    /// For each (i, j), if characters match, `dp[i][j] = dp[i-1][j-1] + dp[i-1][j]`.
    /// Otherwise `dp[i][j] = dp[i-1][j]`. Use u64 to avoid overflow.
    ///
    /// # Complexity
    /// - Time: O(n * m) - filling the DP table
    /// - Space: O(n * m) - the DP table
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (n, m) = (s.len(), t.len());
        let mut dp = vec![vec![0u64; m + 1]; n + 1];

        for row in dp.iter_mut() {
            row[0] = 1;
        }

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j];
                if s[i - 1] == t[j - 1] {
                    dp[i][j] += dp[i - 1][j - 1];
                }
            }
        }

        dp[n][m] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
    }

    #[test]
    fn multiple_paths() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(Solution::num_distinct("a".to_string(), "b".to_string()), 0);
    }

    #[test]
    fn empty_target() {
        assert_eq!(Solution::num_distinct("abc".to_string(), String::new()), 1);
    }

    #[test]
    fn empty_source() {
        assert_eq!(Solution::num_distinct(String::new(), "a".to_string()), 0);
    }

    #[test]
    fn both_empty() {
        assert_eq!(Solution::num_distinct(String::new(), String::new()), 1);
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            Solution::num_distinct("abc".to_string(), "abc".to_string()),
            1
        );
    }

    #[test]
    fn source_shorter_than_target() {
        assert_eq!(
            Solution::num_distinct("ab".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn repeated_characters() {
        assert_eq!(
            Solution::num_distinct("aaa".to_string(), "a".to_string()),
            3
        );
    }
}
