impl Solution {
    /// Find the first word in a sentence that starts with the search word.
    ///
    /// # Intuition
    /// Split the sentence by whitespace and find the first word whose prefix
    /// matches the search word. Return 1-indexed position or -1.
    ///
    /// # Approach
    /// 1. Split sentence into words
    /// 2. Find the first word starting with `search_word`
    /// 3. Return its 1-based index, or -1 if not found
    ///
    /// # Complexity
    /// - Time: O(n · L) where n = word count, L = search word length
    /// - Space: O(1) auxiliary
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split_whitespace()
            .position(|w| w.starts_with(&search_word))
            .map(|i| (i + 1) as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_prefix() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string(),),
            4
        );
    }

    #[test]
    fn first_match() {
        assert_eq!(
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string(),
            ),
            2
        );
    }

    #[test]
    fn not_found() {
        assert_eq!(
            Solution::is_prefix_of_word("hello world".to_string(), "xyz".to_string()),
            -1
        );
    }
}
