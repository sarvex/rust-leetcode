impl Solution {
    /// Sliding window with frequency array for longest unique substring.
    ///
    /// # Intuition
    /// A contiguous window of unique characters can be maintained by expanding
    /// the right boundary and contracting the left boundary whenever a duplicate
    /// is detected. A fixed-size frequency array gives O(1) character counting.
    ///
    /// # Approach
    /// Convert the string to bytes for direct indexing. Maintain a 128-element
    /// frequency array and two pointers (`window_start`, `window_end`). As
    /// `window_end` advances, increment the frequency of the current byte. If
    /// the frequency exceeds one, shrink the window from the left until the
    /// duplicate is removed. Track the maximum window size throughout.
    ///
    /// # Complexity
    /// - Time: O(n) — each character is visited at most twice
    /// - Space: O(1) — fixed 128-element frequency array
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut frequency = [0u16; 128];
        let mut max_length = 0;
        let mut window_start = 0;

        for (window_end, &current_byte) in bytes.iter().enumerate() {
            frequency[current_byte as usize] += 1;

            while frequency[current_byte as usize] > 1 {
                frequency[bytes[window_start] as usize] -= 1;
                window_start += 1;
            }

            max_length = max_length.max(window_end - window_start + 1);
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeating_pattern() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn all_identical() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn interleaved_repeats() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::length_of_longest_substring(String::new()), 0);
    }

    #[test]
    fn all_unique() {
        assert_eq!(
            Solution::length_of_longest_substring("abcdef".to_string()),
            6
        );
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }
}
