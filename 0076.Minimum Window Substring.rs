use std::collections::HashMap;

impl Solution {
    /// Sliding window with character frequency tracking for minimum window substring.
    ///
    /// # Intuition
    /// Expand the window to include all required characters, then contract
    /// from the left to find the smallest valid window. A counter tracks
    /// how many characters in `t` are fully satisfied.
    ///
    /// # Approach
    /// Build a frequency map for `t`. Expand the right pointer, updating
    /// the window map and satisfaction counter. When all characters are
    /// satisfied, shrink from the left while maintaining validity, recording
    /// the minimum window found.
    ///
    /// # Complexity
    /// - Time: O(|s| + |t|) — each character visited at most twice
    /// - Space: O(|s| + |t|) — frequency maps
    pub fn min_window(s: String, t: String) -> String {
        let mut need: HashMap<u8, usize> = HashMap::new();
        let mut window: HashMap<u8, usize> = HashMap::new();

        for &b in t.as_bytes() {
            *need.entry(b).or_default() += 1;
        }

        let s_bytes = s.as_bytes();
        let required = t.len();
        let mut satisfied = 0;
        let mut left = 0;
        let mut best_start = 0;
        let mut best_len = usize::MAX;

        for right in 0..s_bytes.len() {
            let ch = s_bytes[right];
            let count = window.entry(ch).or_default();
            *count += 1;
            if *count <= *need.get(&ch).unwrap_or(&0) {
                satisfied += 1;
            }

            while satisfied == required {
                let window_len = right - left + 1;
                if window_len < best_len {
                    best_len = window_len;
                    best_start = left;
                }

                let left_ch = s_bytes[left];
                let left_count = window.get_mut(&left_ch).unwrap();
                if *left_count <= *need.get(&left_ch).unwrap_or(&0) {
                    satisfied -= 1;
                }
                *left_count -= 1;
                left += 1;
            }
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            s[best_start..best_start + best_len].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn exact_match() {
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn no_valid_window() {
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }

    #[test]
    fn entire_string() {
        assert_eq!(
            Solution::min_window("ab".to_string(), "ab".to_string()),
            "ab"
        );
    }
}
