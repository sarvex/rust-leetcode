impl Solution {
    /// Greedy prefix match with suffix-feasibility to find the lexicographically smallest valid sequence.
    ///
    /// # Intuition
    /// A valid sequence picks `m` strictly-increasing indices from `word1` such that the
    /// extracted string is "almost equal" to `word2` (at most one character changed). To make
    /// the index array lexicographically smallest, greedily pick the smallest index that still
    /// leaves the rest of `word2` completable:
    /// - If `w1[i] == w2[p]`, take it — exact matches never spend the single wildcard and
    ///   taking the earliest possible index can only help later positions.
    /// - Otherwise, fire the wildcard at `i` iff the remaining suffix `w2[p+1..]` is still
    ///   matchable *exactly* in `w1[i+1..]`. Doing it at the earliest valid `i` is optimal
    ///   because the smaller index dominates lexicographic comparison.
    ///
    /// # Approach
    /// 1. Precompute `right[k]` = the latest start index in `word1` from which `w2[k..m]` can
    ///    be matched as a subsequence (`-1` = impossible, `right[m] = n` as empty-suffix
    ///    sentinel). A single right-to-left pass fills this in `O(n)`, and it can early-exit
    ///    the moment all of `word2` is consumed (`j == 0`).
    /// 2. Walk `word1` left-to-right, tracking `p` = number of `word2` chars matched so far:
    ///    - Exact match: record `i`, advance `p`.
    ///    - Mismatch with `right[p + 1] >= i + 1`: fire the wildcard at `i`, then greedily
    ///      match the remaining suffix of `word2` exactly in the rest of `word1` and stop.
    ///
    /// # Complexity
    /// - Time: `O(n + m)` — one right-to-left scan plus one left-to-right scan, each at most
    ///   `n` steps; the suffix array is accessed in `O(1)` per step.
    /// - Space: `O(m)` auxiliary (the `right` array) plus the `m`-sized result. This is a
    ///   strict improvement over an `O(n)` suffix-count array, and the right-to-left early
    ///   exit also skips the untouched prefix when `word2` is much shorter than `word1`.
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let n = w1.len();
        let m = w2.len();

        if m > n {
            return Vec::new();
        }

        // right[k] = the latest index i in word1 such that word2[k..m] matches as a
        // subsequence starting at i. -1 means impossible; right[m] = n is the empty-suffix
        // sentinel that always allows "match nothing" from any position.
        let mut right = vec![-1i32; m + 1];
        right[m] = n as i32;
        {
            let mut j = m;
            for i in (0..n).rev() {
                if j > 0 && w1[i] == w2[j - 1] {
                    j -= 1;
                    right[j] = i as i32;
                    if j == 0 {
                        break; // All of word2 is matched; nothing more to record.
                    }
                }
            }
        }

        let mut result = vec![0i32; m];
        let mut p = 0usize;
        let mut i = 0usize;

        while i < n && p < m {
            if w1[i] == w2[p] {
                // Exact match — always preferred, keeps the wildcard in reserve.
                result[p] = i as i32;
                p += 1;
                i += 1;
            } else if right[p + 1] >= (i + 1) as i32 {
                // Fire the wildcard here: smallest index for position p that still lets the
                // remaining suffix of word2 be matched exactly in word1[i+1..].
                result[p] = i as i32;
                p += 1;
                i += 1;
                // Remaining chars of word2 must be matched exactly (wildcard spent).
                while i < n && p < m {
                    if w1[i] == w2[p] {
                        result[p] = i as i32;
                        p += 1;
                    }
                    i += 1;
                }
                break;
            } else {
                i += 1;
            }
        }

        if p == m { result } else { Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        // [0,1,2]: change word1[0]='v'->'a', keep 'b','c'
        assert_eq!(
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_example2() {
        // [1,2,4]: keep 'a', change word1[2]='c'->'b', keep 'c'
        assert_eq!(
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string()),
            vec![1, 2, 4]
        );
    }

    #[test]
    fn test_example3_no_solution() {
        // "aaaaaa" cannot produce "aaabc" even with one change
        assert_eq!(
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()),
            vec![]
        );
    }

    #[test]
    fn test_example4_exact_match() {
        // "abc" contains "ab" exactly — no wildcard needed
        assert_eq!(
            Solution::valid_sequence("abc".to_string(), "ab".to_string()),
            vec![0, 1]
        );
    }

    #[test]
    fn test_wildcard_beats_later_exact_for_lex_order() {
        // "abcde" / "ace": exact 'a' at 0, then 'b' mismatches 'c'.
        // Wildcard at 1 (change 'b'->'c'), suffix["cde"] can match "e" -> [0,1,4].
        // [0,1,4] < [0,2,4] lexicographically, so wildcard wins.
        assert_eq!(
            Solution::valid_sequence("abcde".to_string(), "ace".to_string()),
            vec![0, 1, 4]
        );
    }

    #[test]
    fn test_single_char_wildcard() {
        // word2 = "b", word1 = "a" — use wildcard on index 0
        assert_eq!(
            Solution::valid_sequence("a".to_string(), "b".to_string()),
            vec![0]
        );
    }

    #[test]
    fn test_wildcard_on_last_char() {
        // "aab" -> "aac": match "aa" exactly at [0,1], change index 2 with wildcard
        assert_eq!(
            Solution::valid_sequence("aab".to_string(), "aac".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn test_wildcard_earlier_than_exact_subsequence() {
        // "abXcd" / "acd": 'a' exact at 0, 'b' mismatches 'c'.
        // Wildcard at 1 ('b'->'c'), then need to match "d" in "Xcd" -> 'd' at 4.
        // [0,1,4] < [0,3,4] (exact subsequence), so wildcard at 1 is lex smaller.
        assert_eq!(
            Solution::valid_sequence("abXcd".to_string(), "acd".to_string()),
            vec![0, 1, 4]
        );
    }

    #[test]
    fn test_wildcard_at_position_zero() {
        // "xabc" / "ac": 'x' mismatches 'a'. suffix[1] covers "abc" which contains "c" -> >=1.
        // Wildcard at 0, then 'c' at 3 => [0,3].
        assert_eq!(
            Solution::valid_sequence("xabc".to_string(), "ac".to_string()),
            vec![0, 3]
        );
    }

    #[test]
    fn test_exact_match_preferred_over_wildcard() {
        // "ghhgghhhhhh" / "gg": 'g' at 0 matches exactly — do NOT spend wildcard.
        // Then 'g' at 1... wait, w1[1]='h' != 'g'. Wildcard at 1? suffix[2] >= 0 -> yes.
        // [0,1] with wildcard change 'h'->'g' is valid and lex smallest.
        assert_eq!(
            Solution::valid_sequence("ghhgghhhhhh".to_string(), "gg".to_string()),
            vec![0, 1]
        );
    }
}
