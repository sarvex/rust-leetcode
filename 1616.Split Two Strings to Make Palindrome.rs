impl Solution {
    /// Two-pointer palindrome check on split string combinations.
    ///
    /// # Intuition
    /// Split a at index i: prefix from a + suffix from b (or vice versa).
    /// Match outer characters from both ends. When they diverge, check if
    /// the remaining middle of either string forms a palindrome.
    ///
    /// # Approach
    /// 1. Two-pointer from edges: match `a[i]` with `b[j]` inward
    /// 2. When mismatch occurs, check if `a[i..=j]` or `b[i..=j]` is a palindrome
    /// 3. Try both orderings (a+b and b+a)
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        fn is_palindrome(s: &[u8], mut i: usize, mut j: usize) -> bool {
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn check(a: &[u8], b: &[u8]) -> bool {
            let (mut i, mut j) = (0, b.len() - 1);
            while i < j && a[i] == b[j] {
                i += 1;
                j -= 1;
            }
            i >= j || is_palindrome(a, i, j) || is_palindrome(b, i, j)
        }

        let (a, b) = (a.as_bytes(), b.as_bytes());
        check(a, b) || check(b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_form_palindrome() {
        assert!(Solution::check_palindrome_formation(
            "x".to_string(),
            "y".to_string(),
        ));
    }

    #[test]
    fn longer_strings() {
        assert!(Solution::check_palindrome_formation(
            "abdef".to_string(),
            "fecab".to_string(),
        ));
    }

    #[test]
    fn cannot_form() {
        assert!(!Solution::check_palindrome_formation(
            "abc".to_string(),
            "def".to_string(),
        ));
    }
}
