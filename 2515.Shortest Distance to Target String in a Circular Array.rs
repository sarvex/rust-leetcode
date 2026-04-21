impl Solution {
    /// Finds the shortest circular distance to a target string from start_index.
    ///
    /// # Intuition
    /// For each matching index, the circular distance is
    /// min(|i - start|, n - |i - start|). A single pass collects all matches
    /// and tracks the minimum distance.
    ///
    /// # Approach
    /// Iterate once through words, comparing each to the target. For every
    /// match, compute the circular distance and keep the running minimum.
    ///
    /// # Complexity
    /// - Time: O(n × m) where m is the target string length
    /// - Space: O(1)
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;

        words
            .iter()
            .enumerate()
            .filter(|(_, w)| **w == target)
            .map(|(i, _)| {
                let d = (i as i32 - start_index).abs();
                d.min(n - d)
            })
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found_nearby() {
        let words = vec![
            "hello".to_string(),
            "i".to_string(),
            "am".to_string(),
            "leetcode".to_string(),
            "hello".to_string(),
        ];
        assert_eq!(Solution::closest_target(words, "hello".to_string(), 1), 1);
    }

    #[test]
    fn test_at_start() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::closest_target(words, "a".to_string(), 0), 0);
    }

    #[test]
    fn test_not_found() {
        let words = vec!["a".to_string(), "b".to_string()];
        assert_eq!(Solution::closest_target(words, "c".to_string(), 0), -1);
    }
}
