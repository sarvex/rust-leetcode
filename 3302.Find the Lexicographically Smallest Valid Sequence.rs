impl Solution {
    /// Greedy prefix match with suffix precomputation to find lexicographically smallest valid sequence.
    ///
    /// # Intuition
    /// We need a subsequence of word1 indices such that the extracted string is "almost equal"
    /// to word2 (at most one character changed). For the lexicographically smallest index
    /// array, we greedily pick the smallest available index at each position: exact matches
    /// are always taken when available, and the one allowed wildcard (mismatch) is fired at
    /// the earliest position where it's necessary and the remaining suffix is still completable.
    ///
    /// # Approach
    /// 1. Precompute `suffix[i]`: the number of characters from the END of word2 that can be
    ///    matched as a subsequence starting from word1[i], using a right-to-left pass.
    ///    `suffix[i] == k` means word2[m-k..] can be greedily matched in word1[i..].
    /// 2. Scan word1 left-to-right, tracking `prefix_len` (exact chars of word2 matched so far).
    /// 3. At each position i:
    ///    - Exact match (w1[i] == w2[prefix_len]): always take it — same index, no wildcard spent.
    ///    - Mismatch: check if wildcard can be placed here. We need suffix[i+1] >= m - prefix_len - 1,
    ///      meaning word2[prefix_len+1..] can still be matched exactly in word1[i+1..].
    ///      If so, place the wildcard at i (earliest mismatch = lex smallest), then greedily
    ///      fill the remaining suffix and stop.
    /// 4. If word1 is exhausted before all m positions are filled, return empty.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n = word1.len(), m = word2.len()
    /// - Space: O(n + m)
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let w1: &[u8] = word1.as_bytes();
        let w2: &[u8] = word2.as_bytes();
        let n = w1.len();
        let m = w2.len();

        // suffix[i] = number of trailing characters of word2 that can be matched
        // as a subsequence in word1[i..n], computed right-to-left.
        let mut suffix = vec![0usize; n + 1];
        let mut j = m;
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1];
            if j > 0 && w1[i] == w2[j - 1] {
                j -= 1;
                suffix[i] = m - j;
            }
        }

        let mut result = vec![0i32; m];
        let mut prefix_len = 0usize;

        for i in 0..n {
            if prefix_len == m {
                break;
            }

            if w1[i] == w2[prefix_len] {
                // Exact match: always preferred — takes the position without spending the wildcard.
                result[prefix_len] = i as i32;
                prefix_len += 1;
            } else {
                // Mismatch: use the wildcard at i if the remaining suffix is still completable.
                // We need word2[prefix_len+1..] (length m - prefix_len - 1) to be matchable
                // in word1[i+1..], which suffix[i+1] >= remaining guarantees.
                let remaining = m - prefix_len - 1;
                if suffix[i + 1] >= remaining {
                    result[prefix_len] = i as i32;
                    prefix_len += 1;

                    // Greedily match the remaining suffix of word2 in word1[i+1..]
                    let mut k = i + 1;
                    while prefix_len < m {
                        if w1[k] == w2[prefix_len] {
                            result[prefix_len] = k as i32;
                            prefix_len += 1;
                        }
                        k += 1;
                    }
                    break;
                }
            }
        }

        if prefix_len == m {
            result
        } else {
            vec![]
        }
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
