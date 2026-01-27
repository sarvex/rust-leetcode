use std::collections::HashMap;

impl Solution {
    /// Counts pairs of strings that use the same set of characters.
    ///
    /// # Intuition
    /// Two strings are similar iff their character sets are identical. A bitmask
    /// of 26 bits encodes each character set uniquely.
    ///
    /// # Approach
    /// 1. For each word, compute a bitmask of its character set
    /// 2. Count previously seen words with the same bitmask
    /// 3. Accumulate pair counts using a hashmap
    ///
    /// # Complexity
    /// - Time: O(n × m) where m is average word length
    /// - Space: O(n) — hashmap of bitmasks
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut counts: HashMap<u32, i32> = HashMap::with_capacity(words.len());

        words.iter().fold(0, |ans, word| {
            let mask = word.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')));
            let pairs = *counts.get(&mask).unwrap_or(&0);
            *counts.entry(mask).or_insert(0) += 1;
            ans + pairs
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_pairs() {
        let words = vec![
            "aba".to_string(),
            "aabb".to_string(),
            "abcd".to_string(),
            "bac".to_string(),
            "aabc".to_string(),
        ];
        assert_eq!(Solution::similar_pairs(words), 2);
    }

    #[test]
    fn test_all_same() {
        let words = vec!["aaa".to_string(), "a".to_string(), "aa".to_string()];
        assert_eq!(Solution::similar_pairs(words), 3);
    }

    #[test]
    fn test_no_pairs() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::similar_pairs(words), 0);
    }
}
