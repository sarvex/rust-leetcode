
impl Solution {
    /// Finds the minimum delete operations to make two strings equal using edit distance DP.
    ///
    /// # Intuition
    /// This is the edit distance problem restricted to deletions only. The DP
    /// recurrence matches characters greedily and counts unmatched deletions.
    ///
    /// # Approach
    /// 1. Build a 2-D DP table where dp[i][j] = min deletions for word1[..i] and word2[..j].
    /// 2. If characters match, no deletion needed (inherit dp[i-1][j-1]).
    /// 3. Otherwise, take the minimum of deleting from either string.
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s = word1.as_bytes();
        let t = word2.as_bytes();
        let (m, n) = (s.len(), t.len());
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if s[i - 1] == t[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    dp[i - 1][j].min(dp[i][j - 1]) + 1
                };
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::min_distance("sea".to_string(), "eat".to_string()),
            2
        );
    }

    #[test]
    fn test_same() {
        assert_eq!(
            Solution::min_distance("abc".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn test_disjoint() {
        assert_eq!(
            Solution::min_distance("abc".to_string(), "def".to_string()),
            6
        );
    }

    #[test]
    fn test_one_empty() {
        assert_eq!(Solution::min_distance("abc".to_string(), "".to_string()), 3);
        assert_eq!(Solution::min_distance("".to_string(), "abc".to_string()), 3);
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_subsequence() {
        // "leetcode" contains "leet" as a subsequence
        assert_eq!(
            Solution::min_distance("leetcode".to_string(), "leet".to_string()),
            4 // delete "code" from first string
        );
    }

    #[test]
    fn test_single_chars() {
        assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0);
        assert_eq!(Solution::min_distance("a".to_string(), "b".to_string()), 2);
    }

    #[test]
    fn test_leetcode_etco() {
        // LCS of "leetcode" and "etco" is "etc" (length 3)
        // Delete: 8 - 3 + 4 - 3 = 6
        assert_eq!(
            Solution::min_distance("leetcode".to_string(), "etco".to_string()),
            6
        );
    }
}
