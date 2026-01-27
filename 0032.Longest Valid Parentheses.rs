impl Solution {
    /// Stack-based tracking for longest valid parentheses substring.
    ///
    /// # Intuition
    /// A stack holding indices of unmatched characters serves as a boundary
    /// marker. When a ')' matches, the distance from the current index to the
    /// new stack top gives the length of the valid substring ending here.
    ///
    /// # Approach
    /// Push -1 as an initial boundary. For '(', push its index. For ')',
    /// pop the stack. If the stack becomes empty, push the current index as
    /// the new boundary. Otherwise, compute the valid length as the difference
    /// between the current index and the new stack top.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass through the string
    /// - Space: O(n) — stack may hold all indices
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack: Vec<i32> = vec![-1];
        let mut max_len = 0;

        for (i, &ch) in bytes.iter().enumerate() {
            let i = i as i32;
            if ch == b'(' {
                stack.push(i);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i);
                } else {
                    max_len = max_len.max(i - stack.last().unwrap());
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_pair() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn nested_and_adjacent() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::longest_valid_parentheses(String::new()), 0);
    }

    #[test]
    fn fully_valid() {
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
    }

    #[test]
    fn all_opening() {
        assert_eq!(Solution::longest_valid_parentheses("(((".to_string()), 0);
    }
}
