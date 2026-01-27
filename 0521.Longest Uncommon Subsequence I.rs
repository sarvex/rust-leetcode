impl Solution {
    /// Returns the longest uncommon subsequence length between two strings.
    ///
    /// # Intuition
    /// If the strings differ, the longer one is itself an uncommon subsequence.
    /// If they are equal, no uncommon subsequence exists.
    ///
    /// # Approach
    /// 1. If a == b, return -1.
    /// 2. Otherwise return the maximum of the two lengths.
    ///
    /// # Complexity
    /// - Time: O(n) for string comparison
    /// - Space: O(1)
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different() {
        assert_eq!(
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string()),
            3
        );
    }

    #[test]
    fn test_same() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()),
            -1
        );
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "a".to_string()),
            3
        );
    }
}
