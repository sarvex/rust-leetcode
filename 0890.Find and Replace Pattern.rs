impl Solution {
    /// Finds words matching a character-mapping pattern.
    ///
    /// # Intuition
    /// Two strings match if there is a bijection between their characters.
    /// Track last-seen positions for both directions of the mapping.
    ///
    /// # Approach
    /// For each word, verify bijection by comparing last-seen positions of
    /// corresponding characters in the word and pattern.
    ///
    /// # Complexity
    /// - Time: O(n * L) where n is word count and L is word length
    /// - Space: O(1) — fixed-size lowercase ASCII lookup tables
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pat = pattern.as_bytes();
        let n = pat.len();
        words
            .into_iter()
            .filter(|word| {
                let wb = word.as_bytes();
                let mut w_map = [None; 26];
                let mut p_map = [None; 26];
                (0..n).all(|i| {
                    let w_idx = (wb[i] - b'a') as usize;
                    let p_idx = (pat[i] - b'a') as usize;
                    let w_prev = w_map[w_idx];
                    let p_prev = p_map[p_idx];
                    w_map[w_idx] = Some(i);
                    p_map[p_idx] = Some(i);
                    w_prev == p_prev
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        let mut result = Solution::find_and_replace_pattern(
            to_vec(&["abc", "deq", "mee", "aqq", "dkd", "ccc"]),
            "abb".to_string(),
        );
        result.sort_unstable();
        assert_eq!(result, to_vec(&["aqq", "mee"]));
    }

    #[test]
    fn test_identity() {
        assert_eq!(
            Solution::find_and_replace_pattern(to_vec(&["a", "b", "c"]), "a".to_string()),
            to_vec(&["a", "b", "c"]),
        );
    }
}
