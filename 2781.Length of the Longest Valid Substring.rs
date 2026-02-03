use std::collections::HashSet;

impl Solution {
    /// Sliding window with a bounded lookback over forbidden lengths.
    ///
    /// # Intuition
    /// Any newly invalid substring must end at the current right index, and every forbidden string
    /// has length at most 10. Only the last 10 characters can introduce a forbidden substring.
    ///
    /// # Approach
    /// Store forbidden strings in a hash set of bytes. Maintain a left boundary for the current
    /// valid window. For each right index, scan substrings ending at right with length up to the
    /// maximum forbidden length. If one is forbidden, move `left` just past its start. Track the
    /// maximum window length seen.
    ///
    /// # Complexity
    /// - Time: O(n * L), where L <= 10
    /// - Space: O(total length of forbidden strings)
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let forbidden_set: HashSet<Vec<u8>> = forbidden
            .into_iter()
            .map(|value| value.into_bytes())
            .collect();
        let max_forbidden_len = forbidden_set
            .iter()
            .map(|value| value.len())
            .max()
            .unwrap_or(0);

        let bytes = word.as_bytes();
        let mut left = 0usize;
        let mut best = 0usize;

        for right in 0..bytes.len() {
            let max_check = max_forbidden_len.min(right + 1);
            for len in 1..=max_check {
                let start = right + 1 - len;
                if start < left {
                    break;
                }
                if forbidden_set.contains(&bytes[start..right + 1]) {
                    left = start + 1;
                    break;
                }
            }

            let current = right + 1 - left;
            if current > best {
                best = current;
            }
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let word = "cbaaaabc".to_string();
        let forbidden = vec!["aaa".to_string(), "cb".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 4);
    }

    #[test]
    fn example_two() {
        let word = "leetcode".to_string();
        let forbidden = vec!["de".to_string(), "le".to_string(), "e".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 4);
    }

    #[test]
    fn single_character_forbidden() {
        let word = "a".to_string();
        let forbidden = vec!["a".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 0);
    }

    #[test]
    fn no_forbidden_match() {
        let word = "abc".to_string();
        let forbidden = vec!["d".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 3);
    }

    #[test]
    fn repeated_forbidden_pair() {
        let word = "aaaaa".to_string();
        let forbidden = vec!["aa".to_string()];
        assert_eq!(Solution::longest_valid_substring(word, forbidden), 1);
    }
}
