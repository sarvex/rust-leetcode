impl Solution {
    /// Merges two strings by alternating characters from each.
    ///
    /// # Intuition
    /// Zip both strings together, then append any leftover characters from
    /// the longer string.
    ///
    /// # Approach
    /// 1. Pre-allocate a result string with combined capacity.
    /// 2. Zip characters and push pairs alternately.
    /// 3. Extend with remaining characters from either string.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m)
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let (mut i1, mut i2) = (word1.chars(), word2.chars());
        loop {
            match (i1.next(), i2.next()) {
                (Some(a), Some(b)) => {
                    result.push(a);
                    result.push(b);
                }
                (Some(a), None) => {
                    result.push(a);
                    result.extend(i1);
                    break;
                }
                (None, Some(b)) => {
                    result.push(b);
                    result.extend(i2);
                    break;
                }
                (None, None) => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_length() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr"
        );
    }

    #[test]
    fn test_word2_longer() {
        assert_eq!(
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs"
        );
    }

    #[test]
    fn test_word1_longer() {
        assert_eq!(
            Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd"
        );
    }
}
