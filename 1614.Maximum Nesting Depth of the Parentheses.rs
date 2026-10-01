impl Solution {
    /// Tracks parenthesis depth on one scan and keeps the maximum.
    ///
    /// # Intuition
    /// In a valid parentheses string, nesting depth is the largest number of
    /// unmatched opening parentheses seen while reading left to right. Digits
    /// and operators never change that count.
    ///
    /// # Approach
    /// 1. Walk each byte once, carrying a running depth.
    /// 2. Increment the depth on `(`, and decrement it on `)`.
    /// 3. Return the maximum depth observed. A balanced string ends at depth 0,
    ///    so the peak is always produced by an opening parenthesis.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_depth(s: String) -> i32 {
        s.bytes()
            .scan(0, |depth, byte| {
                match byte {
                    b'(' => *depth += 1,
                    b')' => *depth -= 1,
                    _ => {}
                }
                Some(*depth)
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::max_depth("()(())((()()))".to_string()), 3);
    }

    #[test]
    fn test_no_parentheses() {
        assert_eq!(Solution::max_depth("1+2-3*4/5".to_string()), 0);
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::max_depth("()".to_string()), 1);
    }

    #[test]
    fn test_adjacent_pairs() {
        assert_eq!(Solution::max_depth("()()()".to_string()), 1);
    }

    #[test]
    fn test_fully_nested() {
        assert_eq!(Solution::max_depth("(((())))".to_string()), 4);
    }

    #[test]
    fn test_depth_resets_between_groups() {
        assert_eq!(Solution::max_depth("(()())((()))".to_string()), 3);
    }
}
