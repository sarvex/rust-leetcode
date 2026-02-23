use std::collections::HashMap;

impl Solution {
    /// Counts prefix buckets of length `k` and returns how many have size at least 2.
    ///
    /// # Intuition
    /// Two words are connected exactly when their first `k` characters are equal.
    /// That means each connected group is fully determined by a single `k`-length
    /// prefix, so we only need frequencies of those prefixes.
    ///
    /// # Approach
    /// - Convert `k` to `usize` (return `0` for invalid negative input).
    /// - Iterate over all words with length at least `k`.
    /// - Take each word's `k`-length prefix and count it in a hash map.
    /// - Count how many map entries have frequency `>= 2`.
    ///
    /// Words shorter than `k` are ignored by construction.
    ///
    /// # Complexity
    /// - Time: O(n * k), where `n = words.len()`.
    /// - Space: O(n) in the worst case for distinct prefixes.
    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let Ok(k) = usize::try_from(k) else {
            return 0;
        };

        let mut prefix_count: HashMap<&str, i32> = HashMap::with_capacity(words.len());

        words
            .iter()
            .filter(|word| word.len() >= k)
            .map(|word| &word[..k])
            .for_each(|prefix| *prefix_count.entry(prefix).or_insert(0) += 1);

        prefix_count.values().filter(|&&count| count >= 2).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let words = vec!["apple", "apply", "banana", "bandit"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::prefix_connected(words, 2), 2);
    }

    #[test]
    fn test_example_2() {
        let words = vec!["car", "cat", "cartoon"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::prefix_connected(words, 3), 1);
    }

    #[test]
    fn test_example_3() {
        let words = vec!["bat", "dog", "dog", "doggy", "bat"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::prefix_connected(words, 3), 2);
    }

    #[test]
    fn test_ignore_words_shorter_than_k() {
        let words = vec!["a", "ab", "abc", "abx", "zzz"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::prefix_connected(words, 3), 0);
    }

    #[test]
    fn test_all_words_form_one_group() {
        let words = vec!["dog", "doge", "dogma", "dog"]
            .into_iter()
            .map(str::to_string)
            .collect();
        assert_eq!(Solution::prefix_connected(words, 3), 1);
    }
}
