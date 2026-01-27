impl Solution {
    /// KMP (Knuth-Morris-Pratt) algorithm for first substring occurrence.
    ///
    /// # Intuition
    /// The KMP failure function precomputes the longest proper prefix that is
    /// also a suffix for each position in the needle. This allows skipping
    /// redundant comparisons when a mismatch occurs during the search.
    ///
    /// # Approach
    /// Build the failure (next) array for the needle using self-matching.
    /// Then scan the haystack, advancing the needle pointer on matches and
    /// falling back via the failure array on mismatches. Return the start
    /// index when the entire needle is matched, or -1 if no match is found.
    ///
    /// # Complexity
    /// - Time: O(m + n) — linear in both haystack and needle lengths
    /// - Space: O(n) — failure array for the needle
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let (m, n) = (haystack.len(), needle.len());

        if n == 0 {
            return 0;
        }

        let mut failure = vec![0usize; n];
        let mut len = 0;
        for i in 1..n {
            while len > 0 && needle[i] != needle[len] {
                len = failure[len - 1];
            }
            if needle[i] == needle[len] {
                len += 1;
            }
            failure[i] = len;
        }

        let mut j = 0;
        for i in 0..m {
            while j > 0 && haystack[i] != needle[j] {
                j = failure[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == n {
                return (i + 1 - n) as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_at_start() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn not_found() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }

    #[test]
    fn found_in_middle() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn empty_needle() {
        assert_eq!(Solution::str_str("abc".to_string(), String::new()), 0);
    }

    #[test]
    fn exact_match() {
        assert_eq!(Solution::str_str("abc".to_string(), "abc".to_string()), 0);
    }
}
