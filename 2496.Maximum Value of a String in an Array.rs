impl Solution {
    /// Finds the maximum value among strings, where value is the numeric parse or length.
    ///
    /// # Intuition
    /// A string's value is its integer representation if purely numeric, otherwise its length.
    ///
    /// # Approach
    /// Use `parse::<usize>()` which returns `Err` for non-numeric strings, falling back to length.
    ///
    /// # Complexity
    /// - Time: O(n × m) where m is the average string length
    /// - Space: O(1)
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter()
            .map(|s| s.parse::<usize>().unwrap_or(s.len()))
            .max()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_strings() {
        let strs = vec![
            "alic3".to_string(),
            "bob".to_string(),
            "3".to_string(),
            "4".to_string(),
            "00000".to_string(),
        ];
        assert_eq!(Solution::maximum_value(strs), 5);
    }

    #[test]
    fn test_all_numeric() {
        let strs = vec![
            "1".to_string(),
            "01".to_string(),
            "001".to_string(),
            "10".to_string(),
        ];
        assert_eq!(Solution::maximum_value(strs), 10);
    }

    #[test]
    fn test_all_alpha() {
        let strs = vec!["abc".to_string(), "de".to_string()];
        assert_eq!(Solution::maximum_value(strs), 3);
    }
}
