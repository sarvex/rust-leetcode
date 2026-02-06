impl Solution {
    /// Sort-based grouping of Caesar-equivalent words — no HashMap.
    ///
    /// # Intuition
    /// Two words are similar iff their normalized form (each char minus the
    /// first char, mod 26) is identical. Sort signatures and count equal runs.
    ///
    /// # Approach
    /// For short words (m ≤ 13) pack the base-26 signature losslessly into a
    /// u64 and sort the integer array (single-instruction comparisons). For
    /// longer words reuse String buffers via zero-cost `into_bytes()`, normalize
    /// in place, and sort. Count consecutive equal elements incrementally.
    ///
    /// # Complexity
    /// - Time: O(n·m·log n) — sort-dominated; comparison is O(1) for u64 path
    /// - Space: O(n) — one u64 per word or reused byte buffer
    pub fn count_pairs(words: Vec<String>) -> i64 {
        let n = words.len();
        if n <= 1 {
            return 0;
        }
        let m = words[0].len();

        if m <= 13 {
            // 26^13 < 2^64 — lossless packing, no allocation per word
            let mut keys: Vec<u64> = words
                .iter()
                .map(|w| {
                    let b = w.as_bytes();
                    let f = b[0];
                    b.iter().fold(0u64, |acc, c| {
                        acc * 26 + (c.wrapping_sub(f).wrapping_add(26) % 26) as u64
                    })
                })
                .collect();
            keys.sort_unstable();
            let mut ans: i64 = 0;
            let mut run: i64 = 1;
            for i in 1..n {
                if keys[i] == keys[i - 1] {
                    ans += run;
                    run += 1;
                } else {
                    run = 1;
                }
            }
            ans
        } else {
            // m > 13 ⟹ n < 7693; reuse String buffers (into_bytes is zero-cost)
            let mut sigs: Vec<Vec<u8>> = words
                .into_iter()
                .map(|w| {
                    let mut b = w.into_bytes();
                    let f = b[0];
                    for c in &mut b {
                        *c = c.wrapping_sub(f).wrapping_add(26) % 26;
                    }
                    b
                })
                .collect();
            sigs.sort_unstable();
            let mut ans: i64 = 0;
            let mut run: i64 = 1;
            for i in 1..sigs.len() {
                if sigs[i] == sigs[i - 1] {
                    ans += run;
                    run += 1;
                } else {
                    run = 1;
                }
            }
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_fusion_layout() {
        assert_eq!(Solution::count_pairs(to_vec(&["fusion", "layout"])), 1);
    }

    #[test]
    fn test_ab_aa_za_aa() {
        assert_eq!(Solution::count_pairs(to_vec(&["ab", "aa", "za", "aa"])), 2);
    }

    #[test]
    fn test_single_word() {
        assert_eq!(Solution::count_pairs(to_vec(&["abc"])), 0);
    }

    #[test]
    fn test_all_same_normalized() {
        assert_eq!(Solution::count_pairs(to_vec(&["aa", "bb", "zz"])), 3);
    }
}
