impl Solution {
    /// Monotonic stack with greedy duplicate removal for lexicographic minimization
    ///
    /// # Intuition
    /// To get the lexicographically smallest string, use a monotonic stack approach.
    /// Pop larger characters if they have remaining occurrences later, keeping characters
    /// that contribute to the smallest lexicographic order.
    ///
    /// # Approach
    /// 1. Build result using monotonic stack: pop larger chars if they appear later
    /// 2. Track character counts both remaining in input and already in stack
    /// 3. Post-process to remove trailing duplicates: keep a character only if it is the
    ///    last occurrence we must keep or a strictly larger unkept character follows
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of s
    /// - Space: O(n) for the result buffers
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return String::new();
        }

        let mut remaining = [0u32; 26];
        bytes
            .iter()
            .for_each(|&b| remaining[(b - b'a') as usize] += 1);

        let mut in_stack = [0u32; 26];
        let mut result: Vec<u8> = Vec::with_capacity(bytes.len());

        for &b in bytes {
            let idx = (b - b'a') as usize;
            remaining[idx] -= 1;

            while let Some(&top) = result.last() {
                let top_idx = (top - b'a') as usize;
                if top > b && (remaining[top_idx] > 0 || in_stack[top_idx] > 1) {
                    result.pop();
                    in_stack[top_idx] -= 1;
                } else {
                    break;
                }
            }

            result.push(b);
            in_stack[idx] += 1;
        }

        let mut result_remaining = [0u32; 26];
        let mut remaining_mask = 0u32;
        for &b in &result {
            let idx = (b - b'a') as usize;
            if result_remaining[idx] == 0 {
                remaining_mask |= 1u32 << idx;
            }
            result_remaining[idx] += 1;
        }

        let mut kept_mask = 0u32;
        let mut write = 0usize;
        let result_len = result.len();

        for i in 0..result_len {
            let b = result[i];
            let idx = (b - b'a') as usize;
            let bit = 1u32 << idx;

            result_remaining[idx] -= 1;
            if result_remaining[idx] == 0 {
                remaining_mask &= !bit;
            }

            let must_keep = result_remaining[idx] == 0 && (kept_mask & bit == 0);
            let larger_unkept_after = (remaining_mask & (!0u32 << (idx + 1))) != 0;

            if must_keep || larger_unkept_after {
                result[write] = b;
                write += 1;
                kept_mask |= bit;
                remaining_mask &= !bit;
            }
        }

        result.truncate(write);
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_pair_with_unique() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aaccb".to_string()),
            "aacb"
        );
    }

    #[test]
    fn test_single_character() {
        assert_eq!(Solution::lex_smallest_after_deletion("z".to_string()), "z");
    }

    #[test]
    fn test_two_identical_chars() {
        assert_eq!(Solution::lex_smallest_after_deletion("aa".to_string()), "a");
    }

    #[test]
    fn test_alternating_pattern() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aba".to_string()),
            "ab"
        );
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aaaa".to_string()),
            "a"
        );
    }

    #[test]
    fn test_no_duplicates_ascending() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_no_duplicates_descending() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("cba".to_string()),
            "cba"
        );
    }

    #[test]
    fn test_complex_reordering() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bcabc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_no_removal_needed() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abac".to_string()),
            "abac"
        );
    }

    #[test]
    fn test_leading_duplicates_with_unique_suffix() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bbca".to_string()),
            "bbca"
        );
    }

    #[test]
    fn test_triple_same() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aaa".to_string()),
            "a"
        );
    }

    #[test]
    fn test_leading_duplicates_then_smaller() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bbac".to_string()),
            "bac"
        );
    }

    #[test]
    fn test_wrap_around_duplicates() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abcab".to_string()),
            "abc"
        );
    }
}
