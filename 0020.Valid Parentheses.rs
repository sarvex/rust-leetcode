impl Solution {
    /// Stack-based bracket matching for parenthesis validation.
    ///
    /// # Intuition
    /// Every opening bracket must be closed by the same type in the correct
    /// order, so the problem is a textbook last-in-first-out match. Push the
    /// *expected closer* for each opener and compare on every closing byte.
    ///
    /// # Approach
    /// 1. Reject odd-length inputs immediately — they can never balance.
    /// 2. Walk the byte slice (ASCII only, so no UTF-8 decoding needed).
    /// 3. For each opener push its matching closer onto the stack.
    /// 4. For each closer, pop and compare; any mismatch (or empty stack)
    ///    means the string is invalid.
    /// 5. Prune aggressively: if the stack ever contains more unmatched
    ///    openers than there are remaining bytes, no suffix can balance it.
    /// 6. After the scan the stack must be empty.
    ///
    /// # Complexity
    /// - Time: O(n) — a single pass over the byte slice with O(1) work per byte.
    /// - Space: O(n) — the stack holds at most ⌈n/2⌉ bytes for valid inputs.
    pub fn is_valid(s: String) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();

        // Odd length can never balance; skip allocation entirely.
        if n & 1 == 1 {
            return false;
        }

        // Tight upper bound for valid inputs; `+ 1` avoids a resize on
        // malformed strings that briefly exceed n/2 before failing.
        let mut stack: Vec<u8> = Vec::with_capacity(n / 2 + 1);

        for (i, &b) in bytes.iter().enumerate() {
            match b {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                b')' | b']' | b'}' => {
                    if stack.pop() != Some(b) {
                        return false;
                    }
                }
                _ => return false,
            }

            // Each item on the stack needs its own closing byte later; if
            // more items remain than remaining bytes, we can give up now.
            if stack.len() > n - i - 1 {
                return false;
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
    fn deeply_nested() {
        assert!(Solution::is_valid("((((((((((()))))))))))".to_string()));
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

    #[test]
    fn odd_length_short_circuits() {
        assert!(!Solution::is_valid("(((".to_string()));
        assert!(!Solution::is_valid("({[".to_string()));
    }

    #[test]
    fn all_openers_cannot_balance() {
        // Even length, but no closers at all — the pruning branch kills it.
        assert!(!Solution::is_valid("((((".to_string()));
        assert!(!Solution::is_valid("[[{{".to_string()));
    }

    #[test]
    fn closer_before_opener() {
        assert!(!Solution::is_valid("}{".to_string()));
        assert!(!Solution::is_valid("][".to_string()));
    }

    #[test]
    fn alternating_valid() {
        assert!(Solution::is_valid("([]){[()]}".to_string()));
    }
}
