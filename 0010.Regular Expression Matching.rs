impl Solution {
    /// Top-down memoized recursion for regular expression matching.
    ///
    /// # Intuition
    /// A pattern character matches the string character directly or via '.'.
    /// The '*' quantifier requires exploring both "skip" (zero occurrences)
    /// and "consume" (one occurrence then continue) branches. Memoization
    /// avoids recomputing overlapping subproblems.
    ///
    /// # Approach
    /// Convert both strings to byte slices for O(1) indexing. Use a 2D memo
    /// table where `memo[i][j]` caches whether `s[i..]` matches `p[j..]`.
    /// For each state, check if the next pattern character is '*' to branch
    /// into skip or consume paths; otherwise match single characters directly.
    ///
    /// # Complexity
    /// - Time: O(m × n) — each state computed at most once
    /// - Space: O(m × n) — memoization table
    pub fn is_match(s: String, p: String) -> bool {
        let (s_bytes, p_bytes) = (s.into_bytes(), p.into_bytes());
        let (m, n) = (s_bytes.len(), p_bytes.len());
        let mut memo = vec![vec![0i8; n + 1]; m + 1];

        fn dfs(s: &[u8], p: &[u8], memo: &mut [Vec<i8>], i: usize, j: usize) -> bool {
            if j >= p.len() {
                return i == s.len();
            }
            if memo[i][j] != 0 {
                return memo[i][j] == 1;
            }

            let first_match = i < s.len() && (s[i] == p[j] || p[j] == b'.');
            let result = if j + 1 < p.len() && p[j + 1] == b'*' {
                dfs(s, p, memo, i, j + 2) || (first_match && dfs(s, p, memo, i + 1, j))
            } else {
                first_match && dfs(s, p, memo, i + 1, j + 1)
            };

            memo[i][j] = if result { 1 } else { -1 };
            result
        }

        dfs(&s_bytes, &p_bytes, &mut memo, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn star_matches_all() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn dot_star() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn complex_pattern() {
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    fn empty_pattern() {
        assert!(!Solution::is_match("a".to_string(), String::new()));
    }

    #[test]
    fn both_empty() {
        assert!(Solution::is_match(String::new(), String::new()));
    }
}
