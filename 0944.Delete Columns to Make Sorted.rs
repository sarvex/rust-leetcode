impl Solution {
    /// Counts columns that are not sorted in a grid of strings.
    ///
    /// # Intuition
    /// A column is unsorted if any adjacent pair of rows has a character
    /// decrease.
    ///
    /// # Approach
    /// For each column, check if all consecutive rows are non-decreasing.
    /// Count columns that fail this check.
    ///
    /// # Complexity
    /// - Time: O(n * m) where n is row count and m is string length
    /// - Space: O(1)
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let rows: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let m = rows[0].len();
        (0..m)
            .filter(|&j| rows.windows(2).any(|w| w[0][j] > w[1][j]))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::min_deletion_size(to_vec(&["cba", "daf", "ghi"])),
            1
        );
    }

    #[test]
    fn test_none_deleted() {
        assert_eq!(Solution::min_deletion_size(to_vec(&["a", "b"])), 0);
    }

    #[test]
    fn test_all_deleted() {
        assert_eq!(
            Solution::min_deletion_size(to_vec(&["zyx", "wvu", "tsr"])),
            3
        );
    }
}
