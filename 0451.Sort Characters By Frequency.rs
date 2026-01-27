impl Solution {
    /// Sorts characters by descending frequency using a frequency array.
    ///
    /// # Intuition
    /// Count each character's occurrences, sort by frequency, and rebuild the
    /// string by repeating each character according to its count.
    ///
    /// # Approach
    /// 1. Count frequencies in a 128-element array.
    /// 2. Collect (count, byte) pairs, sort descending by count.
    /// 3. Build the result by repeating each character.
    ///
    /// # Complexity
    /// - Time: O(n + k log k) where k = 128
    /// - Space: O(n)
    pub fn frequency_sort(s: String) -> String {
        let mut freq = [0usize; 128];
        for &b in s.as_bytes() {
            freq[b as usize] += 1;
        }
        let mut pairs: Vec<(usize, u8)> = freq
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .map(|(i, &count)| (count, i as u8))
            .collect();
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut result = String::with_capacity(s.len());
        for (count, byte) in pairs {
            for _ in 0..count {
                result.push(byte as char);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let result = Solution::frequency_sort("tree".to_string());
        assert!(result == "eert" || result == "eetr");
    }

    #[test]
    fn test_case_sensitive() {
        let result = Solution::frequency_sort("Aabb".to_string());
        assert!(
            result == "bbAa"
                || result == "bbaA"
                || result == "aAbB"
                || result == "aabb"
                || result == "bbaa"
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::frequency_sort("a".to_string()), "a");
    }
}
