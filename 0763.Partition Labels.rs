impl Solution {
    /// Partitions a string so each letter appears in at most one part.
    ///
    /// # Intuition
    /// Record the last occurrence of each character. A partition ends when
    /// the current index reaches the farthest last-occurrence of any
    /// character seen in the current partition.
    ///
    /// # Approach
    /// Precompute last-occurrence array. Scan left-to-right maintaining the
    /// partition boundary as the max last-occurrence seen so far. When the
    /// index reaches this boundary, emit a partition.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 26-element array
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();
        let mut last = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }

        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, &b) in bytes.iter().enumerate() {
            end = end.max(last[(b - b'a') as usize]);
            if i == end {
                result.push((i - start + 1) as i32);
                start = i + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(Solution::partition_labels("abc".to_string()), vec![1, 1, 1]);
    }
}
