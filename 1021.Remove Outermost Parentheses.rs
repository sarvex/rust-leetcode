
impl Solution {
    /// Removes the outermost parentheses of each primitive decomposition.
    ///
    /// # Intuition
    /// Track depth. Outermost parentheses are at depth transitions 0→1 and
    /// 1→0. All other parentheses are kept.
    ///
    /// # Approach
    /// Increment depth on '(' and decrement on ')'. Include the character
    /// only when depth is > 1 after '(' or > 0 before ')'.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the result
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut depth = 0;
        for b in s.bytes() {
            if b == b'(' {
                depth += 1;
                if depth > 1 {
                    result.push('(');
                }
            } else {
                depth -= 1;
                if depth > 0 {
                    result.push(')');
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()"
        );
    }

    #[test]
    fn test_nested() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())"
        );
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
    }
}
