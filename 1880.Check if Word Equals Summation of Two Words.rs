impl Solution {
    /// Checks if two word-values sum to a target word-value.
    ///
    /// # Intuition
    /// Each letter maps to a digit (a=0, b=1, ...). The word's numeric
    /// value is formed by concatenating these digits.
    ///
    /// # Approach
    /// 1. Convert each word to its numeric value by folding over bytes.
    /// 2. Compare the sum of the first two against the target.
    ///
    /// # Complexity
    /// - Time: O(n) where n is total string length
    /// - Space: O(1)
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let to_value =
            |s: &str| -> i64 { s.bytes().fold(0i64, |acc, b| acc * 10 + (b - b'a') as i64) };
        to_value(&first_word) + to_value(&second_word) == to_value(&target_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_sum() {
        assert!(Solution::is_sum_equal(
            "acb".to_string(),
            "cba".to_string(),
            "cdb".to_string()
        ));
    }

    #[test]
    fn test_not_equal() {
        assert!(!Solution::is_sum_equal(
            "aaa".to_string(),
            "a".to_string(),
            "aab".to_string()
        ));
    }
}
