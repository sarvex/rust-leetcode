impl Solution {
    /// Stack-based bracket matching for parenthesis validation.
    ///
    /// # Intuition
    /// Every opening bracket must be closed by the same type in correct order.
    /// A stack naturally enforces last-in-first-out matching: push the expected
    /// closing bracket when an opener is seen, then verify each closer matches
    /// the stack top.
    ///
    /// # Approach
    /// Iterate over characters. For each opening bracket, push its corresponding
    /// closer onto the stack. For each closing bracket, pop the stack and compare.
    /// A mismatch or an empty stack on pop means invalid input. After traversal
    /// the stack must be empty for the string to be valid.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the string
    /// - Space: O(n) — stack may hold all characters in the worst case
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len() / 2);

        for ch in s.bytes() {
            match ch {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                closer => {
                    if stack.pop() != Some(closer) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parentheses() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn mixed_brackets() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn mismatched_types() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn nested_brackets() {
        assert!(Solution::is_valid("{[()]}".to_string()));
    }

    #[test]
    fn unclosed_bracket() {
        assert!(!Solution::is_valid("({".to_string()));
    }

    #[test]
    fn extra_closing() {
        assert!(!Solution::is_valid(")".to_string()));
    }

    #[test]
    fn empty_string() {
        assert!(Solution::is_valid(String::new()));
    }
}
