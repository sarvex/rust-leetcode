use std::collections::HashMap;

impl Solution {
    /// Count prefix-suffix pairs by enumerating KMP borders.
    ///
    /// # Intuition
    /// For any word `w`, a previous word `p` is both a prefix and suffix of `w`
    /// exactly when `p` equals one of `w`'s border strings (including `w` itself).
    ///
    /// # Approach
    /// Process words in order while tracking how many times each previous word
    /// appears. For each word:
    /// - Build the KMP prefix function to obtain all border lengths in O(n).
    /// - Traverse borders starting from full length, then follow `pi[len-1]`.
    /// - Sum frequencies for each border via `&word[..len]`.
    /// - Insert the current word into the frequency map.
    ///
    /// # Complexity
    /// - Time: O(total_length)
    /// - Space: O(total_length)
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut freq: HashMap<String, i64> = HashMap::with_capacity(words.len() * 2);
        let mut answer = 0i64;

        for word in words {
            answer += Self::count_borders(&word, &freq);
            *freq.entry(word).or_insert(0) += 1;
        }

        answer
    }

    fn count_borders(word: &str, freq: &HashMap<String, i64>) -> i64 {
        let bytes = word.as_bytes();
        let n = bytes.len();
        let mut prefix = vec![0usize; n];

        for i in 1..n {
            let mut j = prefix[i - 1];
            while j > 0 && bytes[i] != bytes[j] {
                j = prefix[j - 1];
            }
            if bytes[i] == bytes[j] {
                j += 1;
            }
            prefix[i] = j;
        }

        let mut total = 0i64;
        let mut len = n;
        while len > 0 {
            if let Some(count) = freq.get(&word[..len]) {
                total += *count;
            }
            len = if len == n {
                prefix[n - 1]
            } else {
                prefix[len - 1]
            };
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["a", "aba", "ababa", "aa"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 4);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["pa", "papa", "ma", "mama"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 2);
    }

    #[test]
    fn test_example_3() {
        let words = vec!["abab", "ab"].into_iter().map(String::from).collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
    }

    #[test]
    fn test_all_same_single_char() {
        let words = vec!["a", "a", "a"].into_iter().map(String::from).collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 3);
    }

    #[test]
    fn test_mixed_duplicates() {
        let words = vec!["abc", "ab", "abc", "bc", "abc"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(Solution::count_prefix_suffix_pairs(words), 3);
    }
}
