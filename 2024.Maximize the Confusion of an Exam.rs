impl Solution {
    /// Maximizes consecutive same answers with at most k changes.
    ///
    /// # Intuition
    /// Use a sliding window for each target character. The window expands
    /// while the count of the target (characters to flip) stays within k.
    ///
    /// # Approach
    /// 1. Define a sliding window function for a given flip target.
    /// 2. Track the count of target characters within the window.
    /// 3. Shrink the window when the count exceeds k.
    /// 4. Return the maximum of both target characters.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let bytes = answer_key.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        let max_window = |target: u8| -> usize {
            let mut left = 0;
            let mut count = 0;
            for &b in bytes {
                if b == target {
                    count += 1;
                }
                if count > k {
                    if bytes[left] == target {
                        count -= 1;
                    }
                    left += 1;
                }
            }
            n - left
        };

        max_window(b'T').max(max_window(b'F')) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_true() {
        assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
    }

    #[test]
    fn test_flip_false() {
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::max_consecutive_answers("TTTT".to_string(), 0), 4);
    }
}
