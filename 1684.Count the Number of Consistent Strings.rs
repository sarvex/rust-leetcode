impl Solution {
    /// Bitmask-based allowed character check for consistent strings.
    ///
    /// # Intuition
    /// Encode the allowed characters as a bitmask. A word is consistent if
    /// every character is in the allowed set, checkable via bitwise AND.
    ///
    /// # Approach
    /// 1. Build a boolean mask of allowed characters
    /// 2. Count words where all characters are allowed
    ///
    /// # Complexity
    /// - Time: O(n + total characters in words)
    /// - Space: O(1) — 26-element array
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut mask = [false; 26];
        for b in allowed.bytes() {
            mask[(b - b'a') as usize] = true;
        }

        words
            .iter()
            .filter(|w| w.bytes().all(|b| mask[(b - b'a') as usize]))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn basic_consistent() {
        assert_eq!(
            Solution::count_consistent_strings(
                "ab".to_string(),
                s(&["ad", "bd", "aaab", "baa", "badab"]),
            ),
            2
        );
    }

    #[test]
    fn all_consistent() {
        assert_eq!(
            Solution::count_consistent_strings(
                "abc".to_string(),
                s(&["a", "b", "c", "ab", "ac", "bc", "abc"]),
            ),
            7
        );
    }
}
