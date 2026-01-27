impl Solution {
    /// Brute-force substring matching among array words.
    ///
    /// # Intuition
    /// For each word, check if it appears as a substring of any other word.
    /// The small constraint size makes O(n² · L) acceptable.
    ///
    /// # Approach
    /// 1. For each word, check against all other words for containment
    /// 2. Collect words that are substrings of at least one other word
    ///
    /// # Complexity
    /// - Time: O(n² · L) where L is average word length
    /// - Space: O(1) auxiliary
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .filter(|w| {
                words
                    .iter()
                    .any(|other| !std::ptr::eq(*w, other) && other.contains(w.as_str()))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn basic_matching() {
        let mut result = Solution::string_matching(s(&["mass", "as", "hero", "superhero"]));
        result.sort();
        assert_eq!(result, s(&["as", "hero"]));
    }

    #[test]
    fn no_match() {
        assert_eq!(
            Solution::string_matching(s(&["blue", "green", "bu"])),
            Vec::<String>::new()
        );
    }

    #[test]
    fn nested_substrings() {
        let mut result = Solution::string_matching(s(&["leetcode", "et", "code"]));
        result.sort();
        assert_eq!(result, s(&["code", "et"]));
    }
}
