impl Solution {
    /// Greedy partition with distinct starting characters.
    ///
    /// # Intuition
    /// To maximize the number of substrings, we should make each substring as short
    /// as possible. A single-character substring is ideal when its starting character
    /// hasn't been used yet.
    ///
    /// # Approach
    /// - Use a greedy approach: iterate through the string and create a new substring
    ///   whenever we encounter a character that hasn't been used as a starting character.
    /// - Track used starting characters in a boolean array (size 26 for lowercase English letters).
    /// - When we see a character that's already been used as a start, we extend the current substring.
    /// - When we see a new character, we end the current substring (if any) and start a new one.
    ///
    /// # Complexity
    /// - Time: O(n) - single pass through the string
    /// - Space: O(1) - fixed size boolean array of 26 elements
    pub fn max_distinct(s: String) -> i32 {
        // Use a bitmask instead of boolean array for faster operations
        // Bit i is set if char ('a' + i) has been seen
        let mut mask: u32 = 0;
        let mut count = 0;

        for byte in s.bytes() {
            let bit = 1u32 << (byte - b'a');
            if mask & bit == 0 {
                mask |= bit;
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_distinct("abab".to_string()), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_distinct("abcd".to_string()), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_distinct("aaaa".to_string()), 1);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::max_distinct("a".to_string()), 1);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(Solution::max_distinct("aa".to_string()), 1);
    }

    #[test]
    fn test_two_different() {
        assert_eq!(Solution::max_distinct("ab".to_string()), 2);
    }

    #[test]
    fn test_all_26_letters() {
        assert_eq!(
            Solution::max_distinct("abcdefghijklmnopqrstuvwxyz".to_string()),
            26
        );
    }

    #[test]
    fn test_repeated_pattern() {
        // "abcabc" - can split into "a", "b", "c", "abc" (starts with a, already used)
        // Actually: positions 0(a), 1(b), 2(c) - all distinct = 3 substrings
        assert_eq!(Solution::max_distinct("abcabc".to_string()), 3);
    }

    #[test]
    fn test_alternating() {
        // "ababab" - can only have 2 distinct starting chars: 'a' and 'b'
        assert_eq!(Solution::max_distinct("ababab".to_string()), 2);
    }
}
