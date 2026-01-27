impl Solution {
    /// Removes adjacent duplicates using a stack.
    ///
    /// # Intuition
    /// A stack naturally pairs adjacent duplicates: if the top matches the
    /// current character, pop; otherwise push.
    ///
    /// # Approach
    /// Iterate through characters. Pop when the stack top equals the current
    /// character; otherwise push. Collect the stack into a string.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the stack
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        for b in s.bytes() {
            if stack.last() == Some(&b) {
                stack.pop();
            } else {
                stack.push(b);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
    }

    #[test]
    fn test_all_removed() {
        assert_eq!(Solution::remove_duplicates("aabbcc".to_string()), "");
    }
}
