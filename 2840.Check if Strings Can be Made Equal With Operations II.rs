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
    /// 1. Convert strings to byte slices for direct indexed access, avoiding iterator overhead.
    /// 2. Build frequency-difference arrays for even and odd positions in a single pass,
    ///    processing pairs of indices to eliminate parity computation.
    /// 3. Check both arrays are all-zero via direct array comparison.
    ///
    /// # Complexity
    /// - Time: O(n) — single pass, two elements per iteration
    /// - Space: O(1) — two fixed-size arrays of 26 entries
    pub fn check_strings(s1: String, s2: String) -> bool {
        let (b1, b2) = (s1.as_bytes(), s2.as_bytes());
        let n = b1.len();
        let mut even = [0i32; 26];
        let mut odd = [0i32; 26];

        let mut i = 0;
        while i + 1 < n {
            even[(b1[i] - b'a') as usize] += 1;
            even[(b2[i] - b'a') as usize] -= 1;
            odd[(b1[i + 1] - b'a') as usize] += 1;
            odd[(b2[i + 1] - b'a') as usize] -= 1;
            i += 2;
        }
        if i < n {
            even[(b1[i] - b'a') as usize] += 1;
            even[(b2[i] - b'a') as usize] -= 1;
        }

        even == [0; 26] && odd == [0; 26]
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
