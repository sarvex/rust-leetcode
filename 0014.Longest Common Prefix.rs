impl Solution {
    /// Vertical scanning for longest common prefix across strings.
    ///
    /// # Intuition
    /// The common prefix can be found by comparing characters column by
    /// column across all strings. The first mismatch or end-of-string
    /// determines the prefix length.
    ///
    /// # Approach
    /// Use the first string as a reference. For each character position,
    /// check if all other strings share the same character at that index.
    /// Return the prefix up to the first mismatch or the shortest string's
    /// length.
    ///
    /// # Complexity
    /// - Time: O(S) — where S is the sum of all character comparisons (bounded by min_len × n)
    /// - Space: O(1) — only index variables beyond the output
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let first = strs[0].as_bytes();

        let prefix_len = (0..min_len)
            .take_while(|&i| strs.iter().all(|s| s.as_bytes()[i] == first[i]))
            .count();

        strs[0][..prefix_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_prefix_exists() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }

    #[test]
    fn no_common_prefix() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "");
    }

    #[test]
    fn identical_strings() {
        let strs = vec!["abc".to_string(), "abc".to_string(), "abc".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "abc");
    }

    #[test]
    fn single_string() {
        let strs = vec!["alone".to_string()];
        assert_eq!(Solution::longest_common_prefix(strs), "alone");
    }
}
