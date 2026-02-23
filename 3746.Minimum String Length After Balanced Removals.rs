impl Solution {
    /// Uses count-difference invariance to minimize remaining length.
    ///
    /// # Intuition
    /// Removing a balanced substring deletes the same number of `'a'` and `'b'`,
    /// so the value `count('a') - count('b')` never changes.
    ///
    /// If the final string has length `L`, then `L >= |count('a') - count('b')|`.
    ///
    /// This lower bound is also achievable: while both characters exist, there is
    /// always at least one adjacent boundary between runs, yielding an `"ab"` or
    /// `"ba"` substring (balanced, length 2) that can be removed. Repeating this
    /// removes one `'a'` and one `'b'` each step until one type is exhausted.
    ///
    /// Therefore, the minimum possible length is exactly
    /// `|count('a') - count('b')|`.
    ///
    /// # Approach
    /// - Scan the string once.
    /// - Add `+1` for `'a'` and `-1` for `'b'`.
    /// - Return absolute value of the final balance.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_length_after_removals(s: String) -> i32 {
        s.bytes()
            .fold(0_i32, |balance, ch| {
                balance + if ch == b'a' { 1 } else { -1 }
            })
            .abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_length_after_removals("aabbab".to_string()), 0);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_length_after_removals("aaaa".to_string()), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_length_after_removals("aaabb".to_string()), 1);
    }

    #[test]
    fn test_all_balanced_alternating() {
        assert_eq!(
            Solution::min_length_after_removals("abababab".to_string()),
            0
        );
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::min_length_after_removals("b".to_string()), 1);
    }
}
