/// Subsequence With the Minimum Score
///
/// # Intuition
/// When removing a contiguous segment from t, we keep a prefix and suffix.
/// The key is finding which prefix of t matches a prefix of s, and which
/// suffix of t matches a suffix of s, without overlap in s.
///
/// # Approach
/// 1. Precompute suffix array: for each j, find minimum index in s from which
///    t[j..] can be matched as a subsequence (greedy right-to-left matching)
/// 2. Two-pointer technique: for each prefix length i, find minimum j >= i
///    such that the prefix ends in s before the suffix starts
/// 3. Track minimum score (j - i) across all valid combinations
///
/// # Complexity
/// - Time: O(n + m) - single pass for suffix, two-pointer for main logic
/// - Space: O(n + m) - character vectors and suffix array
impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let n = s.len();
        let m = t.len();

        // suffix[j] = minimum index k in s such that t[j..] is subsequence of s[k..]
        // suffix[m] = n (empty suffix can start at end)
        // suffix[j] = n + 1 means impossible to match
        let mut suffix = vec![n + 1; m + 1];
        suffix[m] = n;

        let mut pos = n;
        for j in (0..m).rev() {
            while pos > 0 && s[pos - 1] != t[j] {
                pos -= 1;
            }
            if pos > 0 {
                pos -= 1;
                suffix[j] = pos;
            }
            // Otherwise suffix[j] remains n + 1 (unmatchable)
        }

        // Two-pointer: find minimum removal score
        let mut ans = m;
        let mut j = 0; // suffix start index in t

        // i = 0: empty prefix, prefix_end = 0
        // Find minimum j where suffix[j] >= 0 (always true for valid suffix[j])
        while j < m && suffix[j] > n {
            j += 1;
        }
        ans = ans.min(j);

        let mut prefix_end = 0; // exclusive end position in s after matching prefix

        for i in 0..m {
            // Extend prefix to include t[i]
            while prefix_end < n && s[prefix_end] != t[i] {
                prefix_end += 1;
            }
            if prefix_end >= n {
                break; // Cannot extend prefix further
            }
            prefix_end += 1; // Move past matched character

            // Ensure j >= i + 1 (we keep t[0..=i] and t[j..])
            j = j.max(i + 1);

            // Find minimum j where suffix[j] >= prefix_end
            while j <= m && suffix[j] < prefix_end {
                j += 1;
            }

            if j <= m {
                ans = ans.min(j - i - 1);
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "abacaba".to_string();
        let t = "bzaa".to_string();
        assert_eq!(Solution::minimum_score(s, t), 1);
    }

    #[test]
    fn test_example_2() {
        let s = "cde".to_string();
        let t = "xyz".to_string();
        assert_eq!(Solution::minimum_score(s, t), 3);
    }

    #[test]
    fn test_t_is_subsequence() {
        let s = "abcde".to_string();
        let t = "ace".to_string();
        assert_eq!(Solution::minimum_score(s, t), 0);
    }

    #[test]
    fn test_single_removal() {
        let s = "abc".to_string();
        let t = "axc".to_string();
        assert_eq!(Solution::minimum_score(s, t), 1);
    }

    #[test]
    fn test_remove_prefix() {
        let s = "abc".to_string();
        let t = "xabc".to_string();
        assert_eq!(Solution::minimum_score(s, t), 1);
    }

    #[test]
    fn test_remove_suffix() {
        let s = "abc".to_string();
        let t = "abcx".to_string();
        assert_eq!(Solution::minimum_score(s, t), 1);
    }

    #[test]
    fn test_identical_strings() {
        let s = "abc".to_string();
        let t = "abc".to_string();
        assert_eq!(Solution::minimum_score(s, t), 0);
    }

    #[test]
    fn test_single_char_match() {
        let s = "a".to_string();
        let t = "a".to_string();
        assert_eq!(Solution::minimum_score(s, t), 0);
    }

    #[test]
    fn test_single_char_no_match() {
        let s = "a".to_string();
        let t = "b".to_string();
        assert_eq!(Solution::minimum_score(s, t), 1);
    }
}
