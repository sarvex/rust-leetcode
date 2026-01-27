impl Solution {
    /// Reverses only the letters in a string, keeping non-letters in place.
    ///
    /// # Intuition
    /// Two pointers from both ends skip non-letters and swap letters.
    ///
    /// # Approach
    /// Convert to bytes. Left pointer advances past non-alpha, right pointer
    /// retreats past non-alpha. When both point to letters, swap them.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the byte buffer (string is consumed)
    pub fn reverse_only_letters(s: String) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len();
        let (mut l, mut r) = (0, n.wrapping_sub(1));
        while l < r && l < n && r < n {
            if !bytes[l].is_ascii_alphabetic() {
                l += 1;
            } else if !bytes[r].is_ascii_alphabetic() {
                r = r.wrapping_sub(1);
            } else {
                bytes.swap(l, r);
                l += 1;
                r = r.wrapping_sub(1);
            }
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_dashes() {
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1teleC-worGn=deba-T!".to_string()
        );
    }
}
