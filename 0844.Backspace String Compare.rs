pub struct Solution;

impl Solution {
    /// Compares two strings with backspace processing using O(1) space.
    ///
    /// # Intuition
    /// Process both strings from right to left, skipping characters erased
    /// by backspaces. Compare surviving characters one by one.
    ///
    /// # Approach
    /// Two pointers scan from the end. For each pointer, count consecutive
    /// '#' characters and skip that many non-'#' characters. Compare the
    /// next surviving characters; mismatch means not equal.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(1)
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (mut i, mut j) = (s.len(), t.len());

        loop {
            let mut skip = 0;
            while i > 0 {
                if s[i - 1] == b'#' {
                    skip += 1;
                } else if skip > 0 {
                    skip -= 1;
                } else {
                    break;
                }
                i -= 1;
            }

            skip = 0;
            while j > 0 {
                if t[j - 1] == b'#' {
                    skip += 1;
                } else if skip > 0 {
                    skip -= 1;
                } else {
                    break;
                }
                j -= 1;
            }

            match (i, j) {
                (0, 0) => return true,
                (0, _) | (_, 0) => return false,
                _ => {
                    if s[i - 1] != t[j - 1] {
                        return false;
                    }
                    i -= 1;
                    j -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_after_backspace() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
    }

    #[test]
    fn test_both_empty() {
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
    }

    #[test]
    fn test_not_equal() {
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }

    #[test]
    fn test_multiple_backspaces() {
        assert!(Solution::backspace_compare(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
    }
}
