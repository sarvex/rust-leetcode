impl Solution {
    /// Find minimum score by removing a contiguous segment from t.
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
    /// - Time: O(n + m)
    /// - Space: O(m)
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();

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
        }

        let mut ans = m;
        let mut j = 0;

        while j < m && suffix[j] > n {
            j += 1;
        }
        ans = ans.min(j);

        let mut prefix_end = 0;
        for i in 0..m {
            while prefix_end < n && s[prefix_end] != t[i] {
                prefix_end += 1;
            }
            if prefix_end >= n {
                break;
            }
            prefix_end += 1;

            j = j.max(i + 1);

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
    fn test_partial_removal() {
        assert_eq!(Solution::minimum_score("abacaba".into(), "bzaa".into()), 1);
    }

    #[test]
    fn test_full_removal() {
        assert_eq!(Solution::minimum_score("cde".into(), "xyz".into()), 3);
    }

    #[test]
    fn test_already_subsequence() {
        assert_eq!(Solution::minimum_score("abcde".into(), "ace".into()), 0);
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(Solution::minimum_score("a".into(), "a".into()), 0);
    }

    #[test]
    fn test_single_char_no_match() {
        assert_eq!(Solution::minimum_score("a".into(), "b".into()), 1);
    }

    #[test]
    fn test_remove_prefix() {
        assert_eq!(Solution::minimum_score("abc".into(), "xabc".into()), 1);
    }

    #[test]
    fn test_remove_suffix() {
        assert_eq!(Solution::minimum_score("abc".into(), "abcx".into()), 1);
    }
}
