impl Solution {
    /// Minimize string length by repeatedly removing "AB" and "CD" substrings.
    ///
    /// # Intuition
    /// Use a stack to greedily remove "AB" and "CD" pairs as they form.
    ///
    /// # Approach
    /// 1. Iterate through bytes
    /// 2. If the top of the stack and current byte form "AB" or "CD", pop
    /// 3. Otherwise, push the current byte
    /// 4. The stack length is the minimum remaining length
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());

        for c in s.bytes() {
            match stack.last() {
                Some(&b'A') if c == b'B' => {
                    stack.pop();
                }
                Some(&b'C') if c == b'D' => {
                    stack.pop();
                }
                _ => stack.push(c),
            }
        }

        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_length("ABFCACDB".into()), 2);
    }

    #[test]
    fn test_no_removal() {
        assert_eq!(Solution::min_length("ACBD".into()), 4);
    }

    #[test]
    fn test_nested() {
        assert_eq!(Solution::min_length("AABB".into()), 2);
    }

    #[test]
    fn test_complete_removal() {
        assert_eq!(Solution::min_length("ABCD".into()), 0);
    }
}
