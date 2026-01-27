impl Solution {
    /// Filters query words that differ from some dictionary word by at most 2 characters.
    ///
    /// # Intuition
    /// For each query, check if any dictionary word matches within 2 character
    /// differences. Since words have equal lengths, this is a simple byte comparison.
    ///
    /// # Approach
    /// 1. For each query, iterate over dictionary words
    /// 2. Count mismatched positions using byte-wise zip comparison
    /// 3. Include the query if any dictionary word has fewer than 3 mismatches
    ///
    /// # Complexity
    /// - Time: O(q * d * m) where q = queries, d = dictionary size, m = word length
    /// - Space: O(q) for the result
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|s| {
                dictionary.iter().any(|t| {
                    s.as_bytes()
                        .iter()
                        .zip(t.as_bytes().iter())
                        .filter(|(&a, &b)| a != b)
                        .count()
                        < 3
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::two_edit_words(
                to_vec(&["word", "note", "ants", "wood"]),
                to_vec(&["wood", "joke", "moat"]),
            ),
            to_vec(&["word", "note", "wood"])
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::two_edit_words(to_vec(&["yes"]), to_vec(&["not"])),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(
            Solution::two_edit_words(to_vec(&["abc"]), to_vec(&["abc"])),
            to_vec(&["abc"])
        );
    }

    #[test]
    fn test_two_edits() {
        assert_eq!(
            Solution::two_edit_words(to_vec(&["abc"]), to_vec(&["axc"])),
            to_vec(&["abc"])
        );
    }
}
