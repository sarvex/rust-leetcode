use std::collections::HashSet;

impl Solution {
    /// Checks sentence similarity using a set of similar word pairs.
    ///
    /// # Intuition
    /// Two sentences are similar if they have equal length and each word pair
    /// is either identical or present in the similarity pairs (in either order).
    ///
    /// # Approach
    /// Store similarity pairs in a `HashSet` of tuples. Zip the two sentences
    /// and verify each pair satisfies the similarity condition.
    ///
    /// # Complexity
    /// - Time: O(p + n) where p is pair count and n is sentence length
    /// - Space: O(p) for the hash set
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        }

        let pairs: HashSet<(&str, &str)> = similar_pairs
            .iter()
            .map(|p| (p[0].as_str(), p[1].as_str()))
            .collect();

        sentence1.iter().zip(sentence2.iter()).all(|(a, b)| {
            a == b
                || pairs.contains(&(a.as_str(), b.as_str()))
                || pairs.contains(&(b.as_str(), a.as_str()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    fn to_pairs(pairs: &[(&str, &str)]) -> Vec<Vec<String>> {
        pairs
            .iter()
            .map(|(a, b)| vec![a.to_string(), b.to_string()])
            .collect()
    }

    #[test]
    fn test_similar() {
        assert!(Solution::are_sentences_similar(
            to_vec(&["great", "acting", "skills"]),
            to_vec(&["fine", "drama", "talent"]),
            to_pairs(&[("great", "fine"), ("drama", "acting"), ("skills", "talent")]),
        ));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!Solution::are_sentences_similar(
            to_vec(&["great"]),
            to_vec(&["great", "acting"]),
            Vec::new(),
        ));
    }

    #[test]
    fn test_identical_words() {
        assert!(Solution::are_sentences_similar(
            to_vec(&["hello"]),
            to_vec(&["hello"]),
            Vec::new(),
        ));
    }
}
