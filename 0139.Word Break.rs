impl Solution {
    /// Determines if a string can be segmented into dictionary words using DP.
    ///
    /// # Intuition
    /// A position i is reachable if some earlier position j is reachable and the
    /// substring s[j..i] exists in the dictionary.
    ///
    /// # Approach
    /// 1. Store dictionary words in a HashSet for O(1) lookup.
    /// 2. Create a boolean DP array where `dp[i]` indicates `s[0..i]` is segmentable.
    /// 3. For each position i, check all possible previous breakpoints j.
    ///
    /// # Complexity
    /// - Time: O(n^2) where n is the string length
    /// - Space: O(n + m) for DP array and dictionary set
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: std::collections::HashSet<String> = word_dict.into_iter().collect();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for i in 1..=s.len() {
            dp[i] = (0..i).any(|j| dp[j] && words.contains(&s[j..i]));
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segmentable_string() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        ));
    }

    #[test]
    fn reusable_words() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        ));
    }

    #[test]
    fn not_segmentable() {
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string(),
            ],
        ));
    }
}
