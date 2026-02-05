impl Solution {
    /// First-occurrence tracking for maximum substring length.
    ///
    /// # Intuition
    /// Record the first occurrence of each character. For subsequent
    /// occurrences, the distance minus one gives the substring length.
    /// Track the maximum across all characters.
    ///
    /// # Approach
    /// 1. Maintain first-seen index per character
    /// 2. For each character, compute distance from first occurrence
    /// 3. Track global maximum
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — 26-element array
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .fold(([-1i32; 26], -1), |(mut first, best), (i, b)| {
                let idx = (b - b'a') as usize;
                if first[idx] == -1 {
                    first[idx] = i as i32;
                    (first, best)
                } else {
                    (first, best.max(i as i32 - first[idx] - 1))
                }
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_equal_chars() {
        assert_eq!(
            Solution::max_length_between_equal_characters("abca".to_string()),
            2
        );
    }

    #[test]
    fn no_equal_chars() {
        assert_eq!(
            Solution::max_length_between_equal_characters("abc".to_string()),
            -1
        );
    }

    #[test]
    fn adjacent_equal() {
        assert_eq!(
            Solution::max_length_between_equal_characters("aa".to_string()),
            0
        );
    }

    #[test]
    fn multiple_chars() {
        assert_eq!(
            Solution::max_length_between_equal_characters("cbzxy".to_string()),
            -1
        );
    }
}
