impl Solution {
    /// Removes stars and their closest left non-star characters using a stack.
    ///
    /// # Intuition
    /// Each star removes the nearest non-removed character to its left, which is
    /// exactly a stack pop operation.
    ///
    /// # Approach
    /// Iterate through bytes: push non-star characters, pop on star characters.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn remove_stars(s: String) -> String {
        let mut stack = String::with_capacity(s.len());
        for &c in s.as_bytes() {
            if c == b'*' {
                stack.pop();
            } else {
                stack.push(c as char);
            }
        }
        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::remove_stars("erase*****".to_string()), "");
    }

    #[test]
    fn test_no_stars() {
        assert_eq!(Solution::remove_stars("hello".to_string()), "hello");
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::remove_stars("a*b*c*".to_string()), "");
    }
}
