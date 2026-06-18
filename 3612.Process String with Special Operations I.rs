impl Solution {
    /// Simulate special string operations using a character buffer.
    ///
    /// # Intuition
    /// Process each character left to right, applying the operation it represents.
    /// A `Vec<u8>` acts as a mutable buffer supporting O(1) push/pop and O(n) reverse/duplicate.
    ///
    /// # Approach
    /// - Lowercase letter: push to buffer
    /// - `*`: pop last byte if non-empty
    /// - `#`: extend buffer with a clone of itself (duplicate)
    /// - `%`: reverse buffer in-place
    ///
    /// # Complexity
    /// - Time: O(n · 2^k) where k is the number of `#` characters (each `#` doubles the buffer)
    /// - Space: O(n · 2^k) for the result buffer
    pub fn process_str(s: String) -> String {
        let mut result: Vec<u8> = Vec::new();
        for byte in s.bytes() {
            match byte {
                b'*' => {
                    result.pop();
                }
                b'#' => {
                    let copy = result.clone();
                    result.extend_from_slice(&copy);
                }
                b'%' => {
                    result.reverse();
                }
                letter => {
                    result.push(letter);
                }
            }
        }
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::process_str("a#b%*".to_string()), "ba".to_string());
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::process_str("z*#".to_string()), "".to_string());
    }

    #[test]
    fn test_only_letters() {
        assert_eq!(Solution::process_str("abc".to_string()), "abc".to_string());
    }

    #[test]
    fn test_remove_from_empty() {
        assert_eq!(Solution::process_str("*".to_string()), "".to_string());
        assert_eq!(Solution::process_str("**a".to_string()), "a".to_string());
    }

    #[test]
    fn test_duplicate_then_reverse() {
        // "ab" -> "#" -> "abab" -> "%" -> "baba"
        assert_eq!(
            Solution::process_str("ab#%".to_string()),
            "baba".to_string()
        );
    }

    #[test]
    fn test_duplicate_empty() {
        assert_eq!(Solution::process_str("#".to_string()), "".to_string());
    }

    #[test]
    fn test_reverse_single() {
        assert_eq!(Solution::process_str("a%".to_string()), "a".to_string());
    }

    #[test]
    fn test_all_ops_combined() {
        // "a" -> "#" -> "aa" -> "b" -> "aab" -> "*" -> "aa" -> "%" -> "aa"
        assert_eq!(Solution::process_str("a#b*%".to_string()), "aa".to_string());
    }
}
