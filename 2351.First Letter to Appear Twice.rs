impl Solution {
    /// Finds the first letter that appears twice using a bitmask.
    ///
    /// # Intuition
    /// Track seen letters with a 26-bit integer. The first collision is the answer.
    ///
    /// # Approach
    /// Iterate through bytes, checking and setting bits for each character index.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn repeated_character(s: String) -> char {
        let mut seen = 0u32;
        for &c in s.as_bytes() {
            let bit = 1 << (c - b'a');
            if seen & bit != 0 {
                return c as char;
            }
            seen |= bit;
        }
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }

    #[test]
    fn test_immediate_repeat() {
        assert_eq!(Solution::repeated_character("aa".to_string()), 'a');
    }

    #[test]
    fn test_last_pair() {
        assert_eq!(
            Solution::repeated_character("abcdefghijklmnopqrstuvwxyza".to_string()),
            'a'
        );
    }
}
