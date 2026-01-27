impl Solution {
    /// Counts asterisks outside pipe-delimited segments.
    ///
    /// # Intuition
    /// Track whether we are inside a pipe-pair using a toggle flag. Only count
    /// asterisks when the flag indicates we are outside.
    ///
    /// # Approach
    /// 1. Iterate through bytes, toggling a boolean on each `|`
    /// 2. Accumulate count only when outside pipe segments
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn count_asterisks(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold((0, true), |(count, outside), &c| match c {
                b'|' => (count, !outside),
                b'*' if outside => (count + 1, outside),
                _ => (count, outside),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_with_segments() {
        assert_eq!(
            Solution::count_asterisks("l]|**et*|c]od*e|*".to_string()),
            2
        );
    }

    #[test]
    fn test_no_asterisks() {
        assert_eq!(Solution::count_asterisks("abc|def|ghi".to_string()), 0);
    }

    #[test]
    fn test_all_outside() {
        assert_eq!(Solution::count_asterisks("***".to_string()), 3);
    }

    #[test]
    fn test_all_inside_pipes() {
        assert_eq!(Solution::count_asterisks("|***|".to_string()), 0);
    }

    #[test]
    fn test_multiple_segments() {
        assert_eq!(Solution::count_asterisks("*|*|*|*|*".to_string()), 3);
    }
}
