/// LeetCode solution container.

impl Solution {
    /// 1D dynamic programming for counting distinct subsequences.
    ///
    /// # Intuition
    /// `dp[j]` represents the number of ways to form `t[0..j]` from the
    /// processed prefix of `s`. If `s[i] == t[j-1]`, we can either use or
    /// skip this character; otherwise we must skip it.
    ///
    /// # Approach
    /// Initialize `dp[0] = 1` (empty t matches any prefix).
    /// For each character in `s`, iterate `j` backward:
    /// if `s[i] == t[j-1]`, then `dp[j] += dp[j-1]`.
    /// Use u64 to avoid overflow.
    ///
    /// # Complexity
    /// - Time: O(n * m) - updating DP state
    /// - Space: O(m) - the DP array
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (n, m) = (s.len(), t.len());
        if m == 0 {
            return 1;
        }
        if n == 0 || m > n {
            return 0;
        }

        let mut dp = vec![0u64; m + 1];
        dp[0] = 1;

        for s_char in s.iter().copied() {
            for (j, t_char) in t.iter().enumerate().rev() {
                if s_char == *t_char {
                    dp[j + 1] += dp[j];
                }
            }
        }

        dp[m] as i32
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
