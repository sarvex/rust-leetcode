impl Solution {
    /// Count words that start with the given prefix.
    ///
    /// # Intuition
    /// A simple filter-and-count using the `starts_with` string method.
    ///
    /// # Approach
    /// Iterate over words and count those whose prefix matches.
    ///
    /// # Complexity
    /// - Time: O(n * p) where n is word count and p is prefix length
    /// - Space: O(1)
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|w| w.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "pay".into(),
                    "attention".into(),
                    "practice".into(),
                    "attend".into()
                ],
                "at".into(),
            ),
            2
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(
            Solution::prefix_count(vec!["abc".into(), "def".into()], "z".into()),
            0
        );
    }
}
