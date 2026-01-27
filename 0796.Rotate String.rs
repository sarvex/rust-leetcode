impl Solution {
    /// Checks if goal is a rotation of s by concatenation trick.
    ///
    /// # Intuition
    /// A rotation of s is always a substring of s + s. If lengths match
    /// and the goal appears in the doubled string, it is a valid rotation.
    ///
    /// # Approach
    /// Verify equal lengths, then check if `(s + s).contains(goal)`.
    ///
    /// # Complexity
    /// - Time: O(n) with efficient substring search
    /// - Space: O(n) for the concatenated string
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && format!("{s}{s}").contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rotation() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
    }

    #[test]
    fn test_not_rotation() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }

    #[test]
    fn test_identical() {
        assert!(Solution::rotate_string("a".to_string(), "a".to_string()));
    }

    #[test]
    fn test_empty() {
        assert!(Solution::rotate_string(String::new(), String::new()));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!Solution::rotate_string("ab".to_string(), "a".to_string()));
    }
}
