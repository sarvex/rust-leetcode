use std::collections::HashMap;

impl Solution {
    /// Longest balanced substring using one HashMap and vec-based two-letter balance.
    ///
    /// A substring is balanced if all distinct characters in it appear the same
    /// number of times (one, two, or three character types).
    ///
    /// # Intuition
    /// - **Single-char**: longest run of the same character (use `chunk_by`).
    /// - **All three equal**: track prefix invariants (ab, bc, ca); when the same
    ///   (ab, bc, ca) appears twice, the substring between has ca = cb = cc.
    /// - **Two-letter balanced**: for each letter x, split by x; in each chunk
    ///   only two letters remain. Use a vis array for prefix balance (one letter +1,
    ///   other -1); same balance twice gives an equal-count substring.
    ///
    /// # Approach
    /// 1. Max over runs from `chunk_by(|a, b| a == b)` for single-char balance.
    /// 2. One HashMap keyed by (ab, bc, ca) with updates: 'a' => ab+=1, ca-=1;
    ///    'b' => ab-=1, bc+=1; 'c' => bc-=1, ca+=1. Same key => balanced substring.
    /// 3. For each x in {'a','b','c'}, split by x; in each chunk use a `vis`
    ///    array (length 2*chunk_len+1) to track first index of each balance;
    ///    recompute balance with one letter +1 and the other -1.
    ///
    /// # Complexity
    /// - Time: O(n) – one pass for runs, one for (ab,bc,ca), then three passes
    ///   over splits with O(chunk) work per chunk (total O(n)).
    /// - Space: O(n) – one HashMap and O(chunk) vis per chunk.
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 2.min(n as i32);
        for ch in s.chunk_by(|a, b| a == b) {
            ans = ans.max(ch.len() as i32);
        }
        let mut ht = HashMap::with_capacity(n + 1);
        let mut ab = 0_i32;
        let mut bc = 0_i32;
        let mut ca = 0_i32;
        ht.insert((ab, bc, ca), 0_i32);
        for (i, c) in s.iter().enumerate() {
            let i = (i + 1) as i32;
            if *c == b'a' {
                ab += 1;
                ca -= 1;
            } else if *c == b'b' {
                ab -= 1;
                bc += 1;
            } else {
                bc -= 1;
                ca += 1;
            }
            ht.entry((ab, bc, ca))
                .and_modify(|x| ans = ans.max(i - *x))
                .or_insert(i);
        }
        for x in b'a'..=b'c' {
            for ch in s.split(|c| *c == x) {
                let n = ch.len();
                if n as i32 <= ans {
                    continue;
                }
                let mut vis = vec![-1_i32; n * 2 + 1];
                let mut p = n;
                vis[p] = 0;
                let y = if x == b'a' { b'b' } else { b'a' };
                for (i, c) in ch.iter().enumerate() {
                    let i = (i + 1) as i32;
                    if *c == y {
                        p += 1;
                    } else {
                        p -= 1;
                    }
                    if vis[p] == -1 {
                        vis[p] = i;
                    } else {
                        ans = ans.max(i - vis[p]);
                    }
                }
            }
        }
        ans
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
        assert_eq!(Solution::longest_balanced("aabcc".to_string()), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_balanced("a".to_string()), 1);
        assert_eq!(Solution::longest_balanced("b".to_string()), 1);
        assert_eq!(Solution::longest_balanced("c".to_string()), 1);
    }

    #[test]
    fn test_two_chars_balanced() {
        assert_eq!(Solution::longest_balanced("ab".to_string()), 2);
        assert_eq!(Solution::longest_balanced("ba".to_string()), 2);
        assert_eq!(Solution::longest_balanced("abc".to_string()), 3);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_balanced("aaaa".to_string()), 4);
        assert_eq!(Solution::longest_balanced("bbbb".to_string()), 4);
        assert_eq!(Solution::longest_balanced("cccc".to_string()), 4);
    }

    #[test]
    fn test_two_pairs() {
        assert_eq!(Solution::longest_balanced("aabb".to_string()), 4);
        assert_eq!(Solution::longest_balanced("abab".to_string()), 4);
    }

    #[test]
    fn test_three_equal() {
        assert_eq!(Solution::longest_balanced("abcabc".to_string()), 6);
        assert_eq!(Solution::longest_balanced("aaabbbccc".to_string()), 9);
    }

    #[test]
    fn test_mixed_pattern() {
        assert_eq!(Solution::longest_balanced("aabbac".to_string()), 4);
    }

    #[test]
    fn test_edge_no_balanced_beyond_single() {
        assert_eq!(Solution::longest_balanced("abcab".to_string()), 3);
    }

    #[test]
    fn test_large_input() {
        let s = "abc".repeat(33333);
        assert_eq!(Solution::longest_balanced(s), 99999);
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::longest_balanced("abcabcabc".to_string()), 9);
    }

    #[test]
    fn test_imbalanced_prefix_suffix() {
        assert_eq!(Solution::longest_balanced("aaaabcbc".to_string()), 6);
    }

    #[test]
    fn test_only_ab_no_c() {
        assert_eq!(Solution::longest_balanced("ababab".to_string()), 6);
    }

    #[test]
    fn test_only_ac_no_b() {
        assert_eq!(Solution::longest_balanced("acacac".to_string()), 6);
    }

    #[test]
    fn test_only_bc_no_a() {
        assert_eq!(Solution::longest_balanced("bcbcbc".to_string()), 6);
    }
}
