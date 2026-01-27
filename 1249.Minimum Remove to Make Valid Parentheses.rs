impl Solution {
    /// Two-pass counting to remove minimum invalid parentheses.
    ///
    /// # Intuition
    /// First count how many valid closing parentheses exist (matching left-to-right).
    /// Then in a second pass, emit characters while tracking open/close budget
    /// to ensure every parenthesis is matched.
    ///
    /// # Approach
    /// 1. Forward pass: count maximum valid `)` by tracking unmatched `(`
    /// 2. Second pass: greedily include `(` only while budget remains, include
    ///    `)` only when a matching `(` has been emitted
    /// 3. Non-parenthesis characters always pass through
    ///
    /// # Complexity
    /// - Time: O(n) two linear passes
    /// - Space: O(n) for the result buffer
    pub fn min_remove_to_make_valid(s: String) -> String {
        let bytes = s.as_bytes();

        let mut right = {
            let mut left = 0i32;
            let mut right = 0i32;
            for &c in bytes {
                match c {
                    b'(' => left += 1,
                    b')' if right < left => right += 1,
                    _ => {}
                }
            }
            right
        };

        let mut open = 0i32;
        let mut result = Vec::with_capacity(bytes.len());

        for &c in bytes {
            match c {
                b'(' => {
                    if open < right {
                        open += 1;
                        result.push(c);
                    }
                }
                b')' => {
                    if open > 0 && right > 0 {
                        right -= 1;
                        open -= 1;
                        result.push(c);
                    }
                }
                _ => result.push(c),
            }
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_extra_closing() {
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de"
        );
    }

    #[test]
    fn remove_extra_opening() {
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d"
        );
    }

    #[test]
    fn remove_all_closing() {
        assert_eq!(Solution::min_remove_to_make_valid("))(".to_string()), "");
    }

    #[test]
    fn no_parentheses() {
        assert_eq!(Solution::min_remove_to_make_valid("abc".to_string()), "abc");
    }
}
