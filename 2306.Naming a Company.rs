use std::collections::HashSet;

impl Solution {
    /// Counts distinct valid company names from swapping first letters of idea pairs.
    ///
    /// # Intuition
    /// Two ideas can form a valid company name only if swapping their first letters
    /// produces two names not in the original set. Grouping by first letter allows
    /// efficient pairwise comparison.
    ///
    /// # Approach
    /// 1. Group all suffixes (everything after first char) by their first character
    /// 2. For each pair of groups (i, j), count suffixes common to both groups
    /// 3. Valid pairs = (unique in i) × (unique in j) × 2 (for both orderings)
    ///
    /// # Complexity
    /// - Time: O(n × m) where n = ideas.len(), m = average suffix length
    /// - Space: O(n × m) for storing all suffixes in hash sets
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        const ALPHABET_SIZE: usize = 26;

        let mut groups: [HashSet<&str>; ALPHABET_SIZE] = Default::default();

        for idea in &ideas {
            let first_char_idx = (idea.as_bytes()[0] - b'a') as usize;
            let suffix = &idea[1..];
            groups[first_char_idx].insert(suffix);
        }

        let mut result: i64 = 0;

        for i in 0..ALPHABET_SIZE {
            if groups[i].is_empty() {
                continue;
            }

            for j in (i + 1)..ALPHABET_SIZE {
                if groups[j].is_empty() {
                    continue;
                }

                let common_suffix_count = groups[i]
                    .iter()
                    .filter(|suffix| groups[j].contains(*suffix))
                    .count();

                let unique_in_i = groups[i].len() - common_suffix_count;
                let unique_in_j = groups[j].len() - common_suffix_count;

                result += 2 * (unique_in_i as i64) * (unique_in_j as i64);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let ideas = vec![
            "coffee".to_string(),
            "donuts".to_string(),
            "time".to_string(),
            "toffee".to_string(),
        ];
        assert_eq!(Solution::distinct_names(ideas), 6);
    }

    #[test]
    fn test_example_two() {
        let ideas = vec!["lack".to_string(), "back".to_string()];
        assert_eq!(Solution::distinct_names(ideas), 0);
    }

    #[test]
    fn test_no_overlap() {
        let ideas = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(Solution::distinct_names(ideas), 2);
    }

    #[test]
    fn test_single_character_suffixes() {
        let ideas = vec![
            "aa".to_string(),
            "ba".to_string(),
            "ab".to_string(),
            "bb".to_string(),
        ];
        assert_eq!(Solution::distinct_names(ideas), 0);
    }

    #[test]
    fn test_multiple_valid_pairs() {
        let ideas = vec!["aaa".to_string(), "bbb".to_string(), "ccc".to_string()];
        assert_eq!(Solution::distinct_names(ideas), 6);
    }
}
