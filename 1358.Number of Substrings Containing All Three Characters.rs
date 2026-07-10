impl Solution {
    /// Last-occurrence tracking for O(n) single-pass with no inner loop.
    ///
    /// # Intuition
    /// For any position `right`, the earliest valid window start is
    /// `min(last[a], last[b], last[c]) + 1`. Every start index from 0 up to
    /// that position yields a valid substring ending at `right`, so we add
    /// `min(last[a], last[b], last[c]) + 1` to the answer directly.
    ///
    /// # Approach
    /// Keep `last[3]` storing the most recent index where each of 'a', 'b', 'c'
    /// was seen (initialised to -1). For each character, update its entry, then
    /// add `min(last) + 1` to the result. No inner shrink loop needed.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn number_of_substrings(s: String) -> i32 {
        let mut last = [-1i32; 3];
        let mut result = 0i32;

        for (i, b) in s.bytes().enumerate() {
            last[(b - b'a') as usize] = i as i32;
            result += last[0].min(last[1]).min(last[2]) + 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::number_of_substrings("abc".to_string()), 1);
    }

    #[test]
    fn test_all_same_char() {
        assert_eq!(Solution::number_of_substrings("aaa".to_string()), 0);
    }

    #[test]
    fn test_no_c() {
        assert_eq!(Solution::number_of_substrings("aaab".to_string()), 0);
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(Solution::number_of_substrings("abcabcabc".to_string()), 21);
    }
}
