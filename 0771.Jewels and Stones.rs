impl Solution {
    /// Counts stones that are also jewels using fixed ASCII lookup.
    ///
    /// # Intuition
    /// Jewel types form a lookup table. Count stones whose type appears in it.
    ///
    /// # Approach
    /// Mark jewel bytes in a fixed ASCII array, then count matching stone bytes
    /// using iterator filtering.
    ///
    /// # Complexity
    /// - Time: O(j + s) where j is jewels length and s is stones length
    /// - Space: O(1) — fixed-size ASCII lookup array
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut jewel_set = [false; 128];
        jewels.bytes().for_each(|b| jewel_set[b as usize] = true);
        stones.bytes().filter(|b| jewel_set[*b as usize]).count() as i32
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
