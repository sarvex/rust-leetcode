impl Solution {
    /// Computes the final value after increment/decrement operations.
    ///
    /// # Intuition
    /// The middle character of each operation determines the sign.
    ///
    /// # Approach
    /// 1. Fold over operations, adding 1 for '+' and -1 for '-'.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations
            .iter()
            .map(|s| if s.as_bytes()[1] == b'+' { 1 } else { -1 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_operations() {
        assert_eq!(
            Solution::final_value_after_operations(vec![
                "--X".to_string(),
                "X++".to_string(),
                "X++".to_string()
            ]),
            1
        );
    }

    #[test]
    fn test_all_increment() {
        assert_eq!(
            Solution::final_value_after_operations(vec!["++X".to_string(), "X++".to_string()]),
            2
        );
    }
}
