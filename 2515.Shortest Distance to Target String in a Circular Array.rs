impl Solution {
    /// Finds the shortest circular distance to a target string from start_index.
    ///
    /// # Intuition
    /// In a circular array, the distance from start to any index is
    /// min(clockwise_steps, counterclockwise_steps). We check expanding
    /// distances from 0 to n/2.
    ///
    /// # Approach
    /// Iterate distance from 0 to n/2, checking both directions for a match.
    ///
    /// # Complexity
    /// - Time: O(n × m) where m is the target string length (for comparison)
    /// - Space: O(1)
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start = start_index as usize;
        let n = words.len();

        (0..=n / 2)
            .find(|&i| words[(start + n - i) % n] == target || words[(start + i) % n] == target)
            .map(|i| i as i32)
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
        assert_eq!(Solution::closet_target(words, "hello".to_string(), 1), 1);
    }

    #[test]
    fn test_at_start() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::closet_target(words, "a".to_string(), 0), 0);
    }

    #[test]
    fn test_not_found() {
        let words = vec!["a".to_string(), "b".to_string()];
        assert_eq!(Solution::closet_target(words, "c".to_string(), 0), -1);
    }
}
