impl Solution {
    /// Monotonic Stack with Duplicate Tracking
    ///
    /// # Intuition
    /// To get the lexicographically smallest string, we use a monotonic stack approach.
    /// We can pop larger characters if they have remaining occurrences later.
    /// We keep characters that contribute to the smallest lexicographic order.
    ///
    /// # Approach
    /// 1. Use a stack to build the result
    /// 2. For each character, pop larger chars from stack if they appear later
    /// 3. Track which characters we've kept to ensure at least one of each
    /// 4. Post-process to remove trailing duplicates (shorter is better when no larger follows)
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of s
    /// - Space: O(n) for the result
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return String::new();
        }

        // Count occurrences of each character
        let mut remaining = [0u32; 26];
        bytes
            .iter()
            .for_each(|&b| remaining[(b - b'a') as usize] += 1);

        // Track count of each character in result
        let mut in_stack = [0u32; 26];

        // Build result using monotonic stack
        let mut result: Vec<u8> = Vec::with_capacity(n);

        for &b in bytes {
            let idx = (b - b'a') as usize;
            remaining[idx] -= 1;

            // Pop larger characters if they have remaining occurrences OR duplicates in stack
            while let Some(&top) = result.last() {
                let top_idx = (top - b'a') as usize;
                // Pop if: top > current AND (top has remaining occurrences OR multiple in stack)
                if top > b && (remaining[top_idx] > 0 || in_stack[top_idx] > 1) {
                    result.pop();
                    in_stack[top_idx] -= 1;
                } else {
                    break;
                }
            }

            // Add current character
            result.push(b);
            in_stack[idx] += 1;
        }

        // Post-process: scan left-to-right, keep char only if:
        // 1. It's the last occurrence and we haven't kept any, OR
        // 2. There's a strictly larger char after that we haven't kept yet
        let mut final_result: Vec<u8> = Vec::with_capacity(result.len());
        let mut kept = [false; 26];

        // Count remaining occurrences in result (from current position onward, excluding current)
        let mut result_remaining = [0u32; 26];
        result
            .iter()
            .for_each(|&b| result_remaining[(b - b'a') as usize] += 1);

        for &b in &result {
            let idx = (b - b'a') as usize;
            result_remaining[idx] -= 1;

            // Must keep if: this is the last occurrence and we haven't kept any
            let must_keep = result_remaining[idx] == 0 && !kept[idx];

            // Should keep if: there's a strictly larger char remaining that we haven't kept yet
            let larger_unkept_after = ((idx + 1)..26).any(|c| !kept[c] && result_remaining[c] > 0);

            if must_keep || larger_unkept_after {
                final_result.push(b);
                kept[idx] = true;
            }
        }

        String::from_utf8(final_result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aaccb".to_string()),
            "aacb"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::lex_smallest_after_deletion("z".to_string()), "z");
    }

    #[test]
    fn test_two_same_chars() {
        assert_eq!(Solution::lex_smallest_after_deletion("aa".to_string()), "a");
    }

    #[test]
    fn test_aba() {
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
    fn test_no_duplicates() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_no_duplicates_reverse() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("cba".to_string()),
            "cba"
        );
    }

    #[test]
    fn test_complex_case() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bcabc".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_abac() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abac".to_string()),
            "abac"
        );
    }

    #[test]
    fn test_bbca() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bbca".to_string()),
            "bbca"
        );
    }

    #[test]
    fn test_aaa() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("aaa".to_string()),
            "a"
        );
    }

    #[test]
    fn test_bbac() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("bbac".to_string()),
            "bac"
        );
    }

    #[test]
    fn test_abcab() {
        assert_eq!(
            Solution::lex_smallest_after_deletion("abcab".to_string()),
            "abc"
        );
    }
}
