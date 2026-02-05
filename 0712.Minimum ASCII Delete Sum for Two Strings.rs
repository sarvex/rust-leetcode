impl Solution {
    /// Minimum ASCII delete sum to make two strings equal using dynamic programming
    ///
    /// # Intuition
    /// Similar to Longest Common Subsequence (LCS), but instead of maximizing
    /// subsequence length, we minimize the ASCII sum of deleted characters.
    ///
    /// # Approach
    /// Use 2D DP where `dp[i][j]` represents minimum ASCII sum to delete to make
    /// `s1[0..i]` and `s2[0..j]` equal. Optimize space to O(min(m, n)) using 1D array.
    ///
    /// # Complexity
    /// - Time: O(m * n) where m and n are lengths of s1 and s2
    /// - Space: O(min(m, n)) using space-optimized 1D DP
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (s1, s2) = if s1.len() > s2.len() {
            (s2.as_bytes(), s1.as_bytes())
        } else {
            (s1.as_bytes(), s2.as_bytes())
        };

        let m = s1.len();
        let n = s2.len();

        let mut dp = vec![0i32; m + 1];

        // Base case: delete all characters from s1
        for i in 1..=m {
            dp[i] = dp[i - 1] + s1[i - 1] as i32;
        }

        for j in 1..=n {
            let mut prev = dp[0];
            // Base case: delete all characters from s2[0..j]
            dp[0] += s2[j - 1] as i32;

            for i in 1..=m {
                let temp = dp[i];
                if s1[i - 1] == s2[j - 1] {
                    // Characters match, no deletion needed
                    dp[i] = prev;
                } else {
                    // Delete from s1 or s2, take minimum
                    dp[i] = std::cmp::min(dp[i - 1] + s1[i - 1] as i32, dp[i] + s2[j - 1] as i32);
                }
                prev = temp;
            }
        }

        dp[m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sea_eat() {
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
            231
        );
    }

    #[test]
    fn test_delete_leet() {
        assert_eq!(
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
            403
        );
    }

    #[test]
    fn test_empty_equivalent() {
        assert_eq!(
            Solution::minimum_delete_sum("a".to_string(), "b".to_string()),
            195 // 'a' = 97, 'b' = 98
        );
    }

    #[test]
    fn test_identical_strings() {
        assert_eq!(
            Solution::minimum_delete_sum("abc".to_string(), "abc".to_string()),
            0
        );
    }

    #[test]
    fn test_no_common_subsequence() {
        assert_eq!(
            Solution::minimum_delete_sum("ab".to_string(), "cd".to_string()),
            394 // 97 + 98 + 99 + 100
        );
    }
}
