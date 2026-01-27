use std::collections::HashMap;

impl Solution {
    /// Sorted-key hash map grouping for anagram classification.
    ///
    /// # Intuition
    /// Two strings are anagrams if and only if their sorted character
    /// sequences are identical. Using the sorted form as a hash map key
    /// groups all anagrams together.
    ///
    /// # Approach
    /// For each string, sort its bytes to produce a canonical key. Insert
    /// the original string into the hash map under that key. Collect all
    /// values as the grouped result.
    ///
    /// # Complexity
    /// - Time: O(n × k log k) — n strings, each of length k sorted
    /// - Space: O(n × k) — hash map storing all strings
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::with_capacity(strs.len());

        for s in strs {
            let mut key = s.as_bytes().to_vec();
            key.sort_unstable();
            groups.entry(key).or_default().push(s);
        }

        groups.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = Solution::group_anagrams(input);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        assert_eq!(
            result,
            vec![vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"],]
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(
            Solution::group_anagrams(vec!["".to_string()]),
            vec![vec!["".to_string()]]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_string()]),
            vec![vec!["a".to_string()]]
        );
    }
}
