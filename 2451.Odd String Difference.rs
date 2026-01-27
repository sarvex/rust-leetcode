use std::collections::HashMap;

impl Solution {
    /// Finds the word with a unique difference array among the given words.
    ///
    /// # Intuition
    /// The difference array of a word is the sequence of consecutive character
    /// differences. Exactly one word has a different difference array.
    ///
    /// # Approach
    /// 1. Compute the difference array for each word as a vector of deltas
    /// 2. Group words by their difference arrays using a HashMap
    /// 3. Return the word belonging to the group of size 1
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is word count and m is word length
    /// - Space: O(n * m) for storing difference arrays
    pub fn odd_string(words: Vec<String>) -> String {
        let mut groups: HashMap<Vec<i8>, (usize, usize)> = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            let diff: Vec<i8> = word
                .as_bytes()
                .windows(2)
                .map(|w| w[1] as i8 - w[0] as i8)
                .collect();
            let entry = groups.entry(diff).or_insert((0, i));
            entry.0 += 1;
        }

        groups
            .into_values()
            .find(|&(count, _)| count == 1)
            .map(|(_, idx)| words[idx].clone())
            .unwrap_or_default()
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
        assert_eq!(Solution::odd_string(to_vec(&["adc", "wzy", "abc"])), "abc");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::odd_string(to_vec(&["aaa", "bob", "ccc", "ddd"])),
            "bob"
        );
    }

    #[test]
    fn test_three_words() {
        assert_eq!(Solution::odd_string(to_vec(&["ab", "cd", "ba"])), "ba");
    }
}
