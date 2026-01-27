impl Solution {
    /// Memoized DFS for checking if s3 is an interleaving of s1 and s2.
    ///
    /// # Intuition
    /// At position `i` in s1 and `j` in s2, the next character in s3 is at
    /// position `i + j`. Try advancing in s1 or s2 if the character matches.
    /// Memoization avoids recomputing overlapping (i, j) states.
    ///
    /// # Approach
    /// Convert strings to byte slices for O(1) indexing. Use a 2D memo table
    /// (0 = unvisited, 1 = true, -1 = false). For each state (i, j), try
    /// matching s1[i] or s2[j] against s3[i+j]. Cache and return the result.
    ///
    /// # Complexity
    /// - Time: O(n × m) — each state computed at most once
    /// - Space: O(n × m) — memo table
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (n, m) = (s1.len(), s2.len());
        if n + m != s3.len() {
            return false;
        }

        let (b1, b2, b3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut memo = vec![vec![0i8; m + 1]; n + 1];

        fn dfs(i: usize, j: usize, b1: &[u8], b2: &[u8], b3: &[u8], memo: &mut [Vec<i8>]) -> bool {
            if i >= b1.len() && j >= b2.len() {
                return true;
            }
            if memo[i][j] != 0 {
                return memo[i][j] == 1;
            }

            let k = i + j;
            let result = (i < b1.len() && b1[i] == b3[k] && dfs(i + 1, j, b1, b2, b3, memo))
                || (j < b2.len() && b2[j] == b3[k] && dfs(i, j + 1, b1, b2, b3, memo));

            memo[i][j] = if result { 1 } else { -1 };
            result
        }

        dfs(0, 0, b1, b2, b3, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_interleaving() {
        assert!(Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
    }

    #[test]
    fn invalid_interleaving() {
        assert!(!Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
    }

    #[test]
    fn empty_strings() {
        assert!(Solution::is_interleave(
            String::new(),
            String::new(),
            String::new()
        ));
    }

    #[test]
    fn length_mismatch() {
        assert!(!Solution::is_interleave(
            "a".to_string(),
            "b".to_string(),
            "abc".to_string()
        ));
    }
}
