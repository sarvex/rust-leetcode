impl Solution {
    /// Produces the lexicographically smallest good caption with minimum cost.
    ///
    /// # Intuition
    /// A good caption requires every character to appear in runs of ≥ 3.
    /// Segment lengths 3–5 suffice (≥ 6 splits optimally). Backward DP finds
    /// the minimum cost; suffix comparison breaks ties lexicographically.
    ///
    /// # Approach
    /// 1. Precompute prefix sums per character for O(1) segment cost queries.
    /// 2. Backward DP: dp[i] = minimum cost for caption[i..n].
    /// 3. For each position, evaluate all (char, len) candidates with minimum
    ///    cost and select the lexicographically smallest suffix via lazy
    ///    character-by-character comparison.
    /// 4. Reconstruct by following best_choice from position 0.
    ///
    /// # Complexity
    /// - Time: O(n × 26 × 5) for DP, amortized suffix comparison
    /// - Space: O(n × 26) for prefix sums, O(n) for DP
    pub fn min_cost_good_caption(caption: String) -> String {
        let n = caption.len();
        if n < 3 {
            return String::new();
        }

        let s: Vec<i64> = caption
            .as_bytes()
            .iter()
            .map(|&c| (c - b'a') as i64)
            .collect();

        let prefix: Vec<Vec<i64>> = (0..26)
            .map(|c| {
                std::iter::once(0)
                    .chain(s.iter().scan(0i64, move |acc, &x| {
                        *acc += (x - c as i64).abs();
                        Some(*acc)
                    }))
                    .collect()
            })
            .collect();

        let cost = |l: usize, r: usize, c: usize| -> i64 { prefix[c][r + 1] - prefix[c][l] };

        let mut dp = vec![i64::MAX; n + 1];
        let mut best_choice = vec![(26usize, 0usize); n + 1];
        dp[n] = 0;

        for i in (0..n).rev() {
            let remaining = n - i;
            if remaining < 3 {
                continue;
            }

            let candidates: Vec<(usize, usize, i64)> = (3..=remaining.min(5))
                .filter(|&len| dp[i + len] != i64::MAX)
                .flat_map(|len| {
                    let j = i + len;
                    (0..26).map(move |c| (c, len, cost(i, j - 1, c) + dp[j]))
                })
                .collect();

            if candidates.is_empty() {
                continue;
            }

            let min_cost = candidates.iter().map(|x| x.2).min().unwrap();
            dp[i] = min_cost;

            let optimal: Vec<(usize, usize)> = candidates
                .into_iter()
                .filter(|x| x.2 == min_cost)
                .map(|x| (x.0, x.1))
                .collect();

            let best = optimal[1..].iter().fold(optimal[0], |best, &candidate| {
                if Self::is_lex_smaller(i, candidate, best, &best_choice, n) {
                    candidate
                } else {
                    best
                }
            });

            best_choice[i] = best;
        }

        if dp[0] == i64::MAX {
            return String::new();
        }

        let mut result = Vec::with_capacity(n);
        let mut i = 0;

        while i < n {
            let (c, len) = best_choice[i];
            result.extend(std::iter::repeat(b'a' + c as u8).take(len));
            i += len;
        }

        String::from_utf8(result).unwrap()
    }

    fn is_lex_smaller(
        start: usize,
        choice1: (usize, usize),
        choice2: (usize, usize),
        best_choice: &[(usize, usize)],
        n: usize,
    ) -> bool {
        let (mut c1, mut len1) = choice1;
        let (mut c2, mut len2) = choice2;
        let mut pos1 = start;
        let mut pos2 = start;
        let mut offset1 = 0usize;
        let mut offset2 = 0usize;

        loop {
            match (pos1 + offset1 >= n, pos2 + offset2 >= n) {
                (true, true) => return false,
                (true, false) => return true,
                (false, true) => return false,
                _ => {}
            }

            match c1.cmp(&c2) {
                std::cmp::Ordering::Less => return true,
                std::cmp::Ordering::Greater => return false,
                std::cmp::Ordering::Equal => {}
            }

            offset1 += 1;
            offset2 += 1;

            if offset1 >= len1 {
                pos1 += len1;
                offset1 = 0;
                if pos1 < n {
                    (c1, len1) = best_choice[pos1];
                }
            }
            if offset2 >= len2 {
                pos2 += len2;
                offset2 = 0;
                if pos2 < n {
                    (c2, len2) = best_choice[pos2];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_char_alternating() {
        assert_eq!(Solution::min_cost_good_caption("cdcd".to_string()), "cccc");
    }

    #[test]
    fn three_char_mixed() {
        assert_eq!(Solution::min_cost_good_caption("aca".to_string()), "aaa");
    }

    #[test]
    fn too_short_returns_empty() {
        assert_eq!(Solution::min_cost_good_caption("bc".to_string()), "");
    }

    #[test]
    fn already_good_unchanged() {
        assert_eq!(
            Solution::min_cost_good_caption("aaabbb".to_string()),
            "aaabbb"
        );
    }

    #[test]
    fn single_char_returns_empty() {
        assert_eq!(Solution::min_cost_good_caption("a".to_string()), "");
    }

    #[test]
    fn three_distinct_chars_median() {
        assert_eq!(Solution::min_cost_good_caption("abc".to_string()), "bbb");
    }

    #[test]
    fn nine_chars_split_into_segments() {
        assert_eq!(
            Solution::min_cost_good_caption("dlqlawbgd".to_string()),
            "llllddddd"
        );
    }

    #[test]
    fn six_identical_chars_unchanged() {
        assert_eq!(
            Solution::min_cost_good_caption("aaaaaa".to_string()),
            "aaaaaa"
        );
    }
}
