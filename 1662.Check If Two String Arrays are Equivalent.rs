impl Solution {
    /// Concatenation equality check.
    ///
    /// # Intuition
    /// Two string arrays represent the same string if their concatenations
    /// are identical.
    ///
    /// # Approach
    /// 1. Join both arrays and compare
    ///
    /// # Complexity
    /// - Time: O(n + m) total character count
    /// - Space: O(n + m) for the joined strings
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn equal_arrays() {
        assert!(Solution::array_strings_are_equal(
            s(&["ab", "c"]),
            s(&["a", "bc"]),
        ));
    }

    #[test]
    fn not_equal() {
        assert!(!Solution::array_strings_are_equal(
            s(&["a", "cb"]),
            s(&["ab", "c"]),
        ));
    }
}
