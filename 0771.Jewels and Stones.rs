impl Solution {
    /// Counts stones that are also jewels using a hash set.
    ///
    /// # Intuition
    /// Jewel types form a lookup set. Count stones whose type appears in the set.
    ///
    /// # Approach
    /// Collect jewel bytes into a `HashSet`, then count matching stone bytes
    /// using iterator filtering.
    ///
    /// # Complexity
    /// - Time: O(j + s) where j is jewels length and s is stones length
    /// - Space: O(j) for the set
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewel_set: std::collections::HashSet<u8> = jewels.bytes().collect();
        stones.bytes().filter(|b| jewel_set.contains(b)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_case() {
        assert_eq!(
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        );
    }

    #[test]
    fn test_no_jewels() {
        assert_eq!(
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
            0
        );
    }

    #[test]
    fn test_all_jewels() {
        assert_eq!(
            Solution::num_jewels_in_stones("abc".to_string(), "aabbcc".to_string()),
            6
        );
    }
}
