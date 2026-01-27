impl Solution {
    /// Generates all letter case permutations via backtracking.
    ///
    /// # Intuition
    /// Each alphabetic character can be toggled between upper and lower case,
    /// creating a binary choice tree. Digits remain fixed.
    ///
    /// # Approach
    /// Backtrack through the string. At each position, recurse with the
    /// original character. If it is alphabetic, also recurse with the toggled
    /// case (XOR with 32 flips ASCII case).
    ///
    /// # Complexity
    /// - Time: O(2^k * n) where k is the number of letters
    /// - Space: O(n) recursion depth plus O(2^k * n) for results
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn backtrack(chars: &mut Vec<u8>, i: usize, result: &mut Vec<String>) {
            if i == chars.len() {
                result.push(String::from_utf8(chars.clone()).unwrap());
                return;
            }
            backtrack(chars, i + 1, result);
            if chars[i].is_ascii_alphabetic() {
                chars[i] ^= 32;
                backtrack(chars, i + 1, result);
                chars[i] ^= 32;
            }
        }

        let mut chars = s.into_bytes();
        let mut result = Vec::new();
        backtrack(&mut chars, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed() {
        let mut result = Solution::letter_case_permutation("a1b2".to_string());
        result.sort();
        let mut expected = vec!["a1b2", "a1B2", "A1b2", "A1B2"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_only() {
        assert_eq!(
            Solution::letter_case_permutation("12345".to_string()),
            vec!["12345"]
        );
    }

    #[test]
    fn test_single_letter() {
        let mut result = Solution::letter_case_permutation("a".to_string());
        result.sort();
        assert_eq!(result, vec!["A", "a"]);
    }
}
