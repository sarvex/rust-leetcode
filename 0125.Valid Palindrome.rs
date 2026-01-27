impl Solution {
    /// Checks if a string is a palindrome considering only alphanumeric characters.
    ///
    /// # Intuition
    /// Use two pointers converging from both ends, skipping non-alphanumeric characters
    /// and comparing case-insensitively.
    ///
    /// # Approach
    /// 1. Convert the string to lowercase bytes.
    /// 2. Advance left pointer past non-alphanumeric bytes.
    /// 3. Retreat right pointer past non-alphanumeric bytes.
    /// 4. Compare bytes at both pointers; return false on mismatch.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the lowercase conversion
    pub fn is_palindrome(s: String) -> bool {
        let s = s.to_lowercase();
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return true;
        }
        let (mut left, mut right) = (0, bytes.len() - 1);
        while left < right {
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            }
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_with_spaces_and_punctuation() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn not_a_palindrome() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn empty_string_is_palindrome() {
        assert!(Solution::is_palindrome(" ".to_string()));
    }

    #[test]
    fn single_character() {
        assert!(Solution::is_palindrome("a".to_string()));
    }
}
