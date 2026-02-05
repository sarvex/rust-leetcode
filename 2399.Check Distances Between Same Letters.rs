impl Solution {
    /// Checks if distances between same letters match the given distance array.
    ///
    /// # Intuition
    /// For each letter appearing twice, the gap between its two positions must equal
    /// the corresponding value in `distance`. Track first occurrence positions and
    /// verify on the second occurrence.
    ///
    /// # Approach
    /// 1. Record first occurrence index (+1 to distinguish from unset)
    /// 2. On second occurrence, verify gap matches the expected distance
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 26-entry array
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let bytes = s.as_bytes();
        let mut first = [0i32; 26];

        bytes.iter().enumerate().all(|(i, c)| {
            let j = (*c - b'a') as usize;
            let i = i as i32;
            if first[j] > 0 {
                i - first[j] == distance[j]
            } else {
                first[j] = i + 1;
                true
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::check_distances(
            "abaccb".to_string(),
            vec![
                1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::check_distances(
            "aa".to_string(),
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));
    }

    #[test]
    fn test_adjacent_pair() {
        assert!(Solution::check_distances(
            "aa".to_string(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));
    }

    #[test]
    fn test_multiple_pairs() {
        assert!(Solution::check_distances(
            "abba".to_string(),
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ));
    }
}
