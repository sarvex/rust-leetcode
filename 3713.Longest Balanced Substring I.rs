impl Solution {
    /// Expand window with frequency tracking for balanced substring detection.
    ///
    /// # Intuition
    /// For each starting position, expand the substring and track character frequencies.
    /// A substring is balanced when all non-zero frequencies are equal.
    ///
    /// # Approach
    /// - Use nested loops: outer loop for starting index, inner loop for ending index
    /// - Track frequency of each character (26 lowercase letters)
    /// - For each substring, check if all non-zero frequencies are identical
    /// - Early exit: if remaining substring can't beat current max, skip
    ///
    /// # Complexity
    /// - Time: O(n² × 26) = O(n²), where n ≤ 1000
    /// - Space: O(1), fixed size frequency array of 26 elements
    pub fn longest_balanced(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut max_len = 0;

        // Try every starting position
        for start in 0..n {
            // Early termination: remaining substring can't beat current max
            if n - start <= max_len {
                break;
            }

            let mut freq = [0; 26];

            // Expand substring ending at each position from start
            for (end, &byte) in bytes.iter().enumerate().skip(start) {
                let idx = (byte - b'a') as usize;
                freq[idx] += 1;

                // Check if all non-zero frequencies are equal
                if Self::is_balanced(&freq) {
                    max_len = max_len.max(end - start + 1);
                }
            }
        }

        max_len as i32
    }

    /// Check if all non-zero frequencies in the array are equal
    /// O(26) = O(1) time, no allocations
    fn is_balanced(freq: &[i32; 26]) -> bool {
        let mut target = 0;

        for &count in freq.iter() {
            if count > 0 {
                if target == 0 {
                    target = count;
                } else if count != target {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_balanced("zzabccy".to_string()), 4);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_balanced("a".to_string()), 1);
    }

    #[test]
    fn test_all_same_char() {
        assert_eq!(Solution::longest_balanced("aaaa".to_string()), 4);
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(Solution::longest_balanced("abcd".to_string()), 4);
    }

    #[test]
    fn test_no_balanced_longer_than_1() {
        assert_eq!(Solution::longest_balanced("aab".to_string()), 2);
    }

    #[test]
    fn test_complex_pattern() {
        assert_eq!(Solution::longest_balanced("aabbcc".to_string()), 6);
    }
}
