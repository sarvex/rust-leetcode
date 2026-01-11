impl Solution {
    /// 3441. Minimum Cost Good Caption
    ///
    /// # Intuition
    /// A good caption requires every character to appear in groups of at least 3 consecutive
    /// occurrences. We can change characters to adjacent alphabet letters with cost 1 per step.
    /// Since groups must have length ≥ 3, we only need to consider segment lengths 3, 4, 5
    /// (any length ≥ 6 can be optimally split into smaller valid segments).
    ///
    /// # Approach
    /// 1. Use backward DP where dp[i] = minimum cost to make caption[i..n] a good caption
    /// 2. Track best_choice[i] = (char, len) that gives lexicographically smallest suffix
    /// 3. When comparing choices with equal cost, simulate and compare resulting suffixes
    /// 4. Reconstruct by following the precomputed best choices
    ///
    /// # Complexity
    /// - Time: O(n × 26 × 5) for DP with suffix comparisons amortized
    /// - Space: O(n × 26) for prefix sums, O(n) for DP arrays
    pub fn min_cost_good_caption(caption: String) -> String {
        let n = caption.len();
        if n < 3 {
            return String::new();
        }

        let s: Vec<i64> = caption.bytes().map(|c| (c - b'a') as i64).collect();

        // Precompute prefix sums for efficient cost calculation
        let mut prefix = vec![vec![0i64; n + 1]; 26];
        for c in 0..26 {
            for i in 0..n {
                prefix[c][i + 1] = prefix[c][i] + (s[i] - c as i64).abs();
            }
        }

        let cost = |l: usize, r: usize, c: usize| -> i64 { prefix[c][r + 1] - prefix[c][l] };

        // Backward DP
        // dp[i] = minimum cost to make s[i..n] a good caption
        // best_choice[i] = (char, len) giving lex smallest suffix among optimal choices
        let mut dp = vec![i64::MAX; n + 1];
        let mut best_choice = vec![(26usize, 0usize); n + 1]; // (char, len)
        dp[n] = 0;

        for i in (0..n).rev() {
            let remaining = n - i;
            if remaining < 3 {
                continue;
            }

            // Collect all valid (char, len) candidates with their costs
            let mut candidates: Vec<(usize, usize, i64)> = Vec::new();

            for len in 3..=remaining.min(5) {
                let j = i + len;
                if dp[j] == i64::MAX {
                    continue;
                }

                for c in 0..26 {
                    let new_cost = cost(i, j - 1, c) + dp[j];
                    candidates.push((c, len, new_cost));
                }
            }

            if candidates.is_empty() {
                continue;
            }

            // Find minimum cost
            let min_cost = candidates.iter().map(|x| x.2).min().unwrap();
            dp[i] = min_cost;

            // Filter to only minimum cost candidates
            let optimal: Vec<(usize, usize)> = candidates
                .into_iter()
                .filter(|x| x.2 == min_cost)
                .map(|x| (x.0, x.1))
                .collect();

            // Find lexicographically smallest among optimal choices
            let mut best = optimal[0];
            for &candidate in &optimal[1..] {
                if Self::is_lex_smaller(i, candidate, best, &best_choice, n) {
                    best = candidate;
                }
            }

            best_choice[i] = best;
        }

        if dp[0] == i64::MAX {
            return String::new();
        }

        // Reconstruct by following best_choice
        let mut result = Vec::with_capacity(n);
        let mut i = 0;

        while i < n {
            let (c, len) = best_choice[i];
            for _ in 0..len {
                result.push(b'a' + c as u8);
            }
            i += len;
        }

        String::from_utf8(result).unwrap()
    }

    /// Compare two choices (c1, len1) vs (c2, len2) at position i
    /// Returns true if choice1 produces lexicographically smaller suffix
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
            // Both reached end
            if pos1 + offset1 >= n && pos2 + offset2 >= n {
                return false; // Equal
            }
            if pos1 + offset1 >= n {
                return true; // Shorter is considered smaller (shouldn't happen with valid partitions)
            }
            if pos2 + offset2 >= n {
                return false;
            }

            // Compare characters at current position
            if c1 < c2 {
                return true;
            }
            if c1 > c2 {
                return false;
            }

            // Characters equal, advance both
            offset1 += 1;
            offset2 += 1;

            // Move to next segment if needed
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
    fn test_example_1() {
        assert_eq!(Solution::min_cost_good_caption("cdcd".to_string()), "cccc");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_cost_good_caption("aca".to_string()), "aaa");
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_cost_good_caption("bc".to_string()), "");
    }

    #[test]
    fn test_already_good() {
        assert_eq!(
            Solution::min_cost_good_caption("aaabbb".to_string()),
            "aaabbb"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::min_cost_good_caption("a".to_string()), "");
    }

    #[test]
    fn test_length_three() {
        assert_eq!(Solution::min_cost_good_caption("abc".to_string()), "bbb");
    }

    #[test]
    fn test_failing_case() {
        assert_eq!(
            Solution::min_cost_good_caption("dlqlawbgd".to_string()),
            "llllddddd"
        );
    }

    #[test]
    fn test_six_chars() {
        assert_eq!(
            Solution::min_cost_good_caption("aaaaaa".to_string()),
            "aaaaaa"
        );
    }
}
