impl Solution {
    /// Finds the longest substring of consecutive alphabetical characters.
    ///
    /// # Intuition
    /// Scan through adjacent byte pairs. When two consecutive bytes differ by exactly 1,
    /// the alphabetical run continues; otherwise it resets.
    ///
    /// # Approach
    /// 1. Convert to bytes for efficient comparison
    /// 2. Use `windows(2)` to examine consecutive pairs
    /// 3. Fold over pairs, tracking current run length and maximum seen
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn longest_continuous_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        bytes
            .windows(2)
            .fold((1, 1), |(ans, cnt), w| {
                if w[1] == w[0] + 1 {
                    (ans.max(cnt + 1), cnt + 1)
                } else {
                    (ans, 1)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::longest_continuous_substring("abacaba".to_string()),
            2
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::longest_continuous_substring("abcde".to_string()),
            5
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_continuous_substring("z".to_string()), 1);
    }

    #[test]
    fn test_full_alphabet() {
        assert_eq!(
            Solution::longest_continuous_substring("abcdefghijklmnopqrstuvwxyz".to_string()),
            26
        );
    }

    #[test]
    fn test_no_consecutive() {
        assert_eq!(Solution::longest_continuous_substring("zyx".to_string()), 1);
    }
}
