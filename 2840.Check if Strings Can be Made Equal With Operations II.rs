impl Solution {
    /// Checks if two strings can be made equal by swapping characters at same-parity indices.
    ///
    /// # Intuition
    /// When `j - i` is even, indices `i` and `j` share the same parity (both even or
    /// both odd). Through transitive swaps, any character at an even index can reach any
    /// other even index, and similarly for odd indices. So the two strings are equivalent
    /// iff the multiset of characters at even positions matches, and likewise for odd.
    ///
    /// # Approach
    /// 1. Build frequency counts for characters at even indices and odd indices separately
    ///    for both strings.
    /// 2. Compare the two frequency arrays — if both match, return `true`.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass over both strings
    /// - Space: O(1) — two fixed-size arrays of 26 entries
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut freq = [[0i32; 26]; 2];

        s1.bytes()
            .zip(s2.bytes())
            .enumerate()
            .for_each(|(i, (a, b))| {
                let p = i & 1;
                freq[p][(a - b'a') as usize] += 1;
                freq[p][(b - b'a') as usize] -= 1;
            });

        freq.iter().all(|row| row.iter().all(|&v| v == 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::check_strings(
            "abcdba".to_string(),
            "cabdab".to_string()
        ));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::check_strings(
            "abe".to_string(),
            "bea".to_string()
        ));
    }

    #[test]
    fn test_identical_strings() {
        assert!(Solution::check_strings(
            "hello".to_string(),
            "hello".to_string()
        ));
    }

    #[test]
    fn test_single_character() {
        assert!(Solution::check_strings("a".to_string(), "a".to_string()));
        assert!(!Solution::check_strings("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_two_characters() {
        assert!(!Solution::check_strings("ab".to_string(), "ba".to_string()));
        assert!(Solution::check_strings("ab".to_string(), "ab".to_string()));
    }

    #[test]
    fn test_all_same_character() {
        assert!(Solution::check_strings(
            "aaaa".to_string(),
            "aaaa".to_string()
        ));
    }
}
