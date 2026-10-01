impl Solution {
    /// Builds ≤26 candidate windows (one per letter) and runs interval-scheduling.
    ///
    /// # Intuition
    /// Each chosen substring must contain *every* occurrence of every letter it
    /// touches. Starting at a letter `c`'s first position and ending at its
    /// last, we repeatedly swallow every other letter whose `last[..]` sticks
    /// out to the right. If any letter inside the window has its *first*
    /// occurrence to the left of our start, this starting letter is doomed —
    /// no valid window begins at `c`.
    ///
    /// That gives at most 26 candidates with a crucial structural property:
    /// any two valid windows are either disjoint or one fully contains the
    /// other (they cannot partially overlap, because the overlapping letter
    /// would force each window to swallow the other). So the problem collapses
    /// to classic greedy interval scheduling, which simultaneously maximizes
    /// the count and — because nested windows sort before their containers —
    /// minimizes total length.
    ///
    /// # Approach
    /// 1. One pass over `s` to compute `first[c]` and `last[c]` for each
    ///    letter.
    /// 2. For every letter that actually appears, grow a window starting at
    ///    `first[c]`:
    ///    * if some inner letter's `first[..] < l`, mark the candidate invalid
    ///      and stop;
    ///    * otherwise extend `r` to `max(r, last[..])` and keep scanning until
    ///      `i > r`.
    /// 3. Sort the ≤26 valid candidates by right endpoint ascending.
    /// 4. Greedy pass: pick each window whose left end is strictly greater
    ///    than the previously picked window's right end.
    ///
    /// # Complexity
    /// - Time: O(n · Σ) with Σ = 26 — the window expansion visits each index at
    ///   most once per starting letter, so overall O(n) in practice.
    /// - Space: O(Σ) auxiliary; the output stores O(n) bytes in the worst case.
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        const NONE: usize = usize::MAX;

        let bytes = s.as_bytes();
        let n = bytes.len();

        // First and last occurrence of each lowercase letter.
        let mut first = [NONE; 26];
        let mut last = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            let c = (b - b'a') as usize;
            if first[c] == NONE {
                first[c] = i;
            }
            last[c] = i;
        }

        // Candidate windows (left, right) — at most one per present letter.
        let mut windows: Vec<(usize, usize)> = Vec::with_capacity(26);
        for c in 0..26 {
            if first[c] == NONE {
                continue;
            }
            let l = first[c];
            let mut r = last[c];
            let mut i = l;
            let mut valid = true;
            while i <= r {
                let cc = (bytes[i] - b'a') as usize;
                if first[cc] < l {
                    // An inner letter leaks to the left of our start — abandon.
                    valid = false;
                    break;
                }
                if last[cc] > r {
                    r = last[cc];
                }
                i += 1;
            }
            if valid {
                windows.push((l, r));
            }
        }

        // Greedy interval scheduling: sort by right endpoint ascending, then
        // pick each window that starts after the last chosen one ends.
        windows.sort_by_key(|&(_, r)| r);

        let mut result: Vec<String> = Vec::with_capacity(windows.len());
        // `n` means "nothing picked yet"; any `l < n` qualifies on the first pick.
        let mut prev_end = n;
        for (l, r) in windows {
            if prev_end == n || l > prev_end {
                // SAFETY: `bytes` is ASCII lowercase, so every slice is valid UTF-8.
                let piece = std::str::from_utf8(&bytes[l..=r])
                    .expect("input is ASCII lowercase")
                    .to_string();
                result.push(piece);
                prev_end = r;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The problem allows any order; sort so comparisons are deterministic.
    fn normalize(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    /// Validates that `pieces` are:
    /// * non-overlapping,
    /// * each slice of `s`,
    /// * closed under "contains a letter ⇒ contains every occurrence of it".
    fn assert_valid_cover(s: &str, pieces: &[String]) {
        // Non-overlapping and present in s: locate each piece and check order.
        let mut spans: Vec<(usize, usize)> = Vec::with_capacity(pieces.len());
        let mut cursor = 0usize;
        let mut remaining = s;
        // Match pieces in the order they appear by scanning left-to-right.
        let mut sorted_pieces: Vec<&str> = pieces.iter().map(|p| p.as_str()).collect();
        sorted_pieces.sort_by_key(|p| s.find(p).unwrap_or(usize::MAX));
        for piece in sorted_pieces {
            let rel = remaining.find(piece).expect("piece must appear in s");
            let start = cursor + rel;
            let end = start + piece.len() - 1;
            spans.push((start, end));
            cursor = end + 1;
            remaining = &s[cursor..];
        }
        // Verify spans are in increasing, non-overlapping order.
        for pair in spans.windows(2) {
            assert!(pair[0].1 < pair[1].0, "overlapping pieces: {pair:?}");
        }
        // Each piece swallows all occurrences of its letters.
        let bytes = s.as_bytes();
        for &(l, r) in &spans {
            let mut inside = [false; 26];
            for &b in &bytes[l..=r] {
                inside[(b - b'a') as usize] = true;
            }
            for (i, &b) in bytes.iter().enumerate() {
                if inside[(b - b'a') as usize] {
                    assert!(
                        i >= l && i <= r,
                        "letter {} at index {i} leaks outside span {l}..={r}",
                        b as char
                    );
                }
            }
        }
    }

    #[test]
    fn test_example_1() {
        let out = Solution::max_num_of_substrings("adefaddaccc".to_string());
        assert_valid_cover("adefaddaccc", &out);
        assert_eq!(normalize(out), vec!["ccc", "e", "f"]);
    }

    #[test]
    fn test_example_2() {
        let out = Solution::max_num_of_substrings("abbaccd".to_string());
        assert_valid_cover("abbaccd", &out);
        assert_eq!(normalize(out), vec!["bb", "cc", "d"]);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(
            Solution::max_num_of_substrings("a".to_string()),
            vec!["a".to_string()]
        );
    }

    #[test]
    fn test_single_run_of_same_letter() {
        // Only one letter present → one window covering the whole string.
        assert_eq!(
            Solution::max_num_of_substrings("aaaa".to_string()),
            vec!["aaaa".to_string()]
        );
    }

    #[test]
    fn test_each_letter_isolated() {
        // Every letter occupies its own 1-length block, so we get them all.
        let out = Solution::max_num_of_substrings("abcdefg".to_string());
        assert_valid_cover("abcdefg", &out);
        assert_eq!(
            normalize(out),
            vec!["a", "b", "c", "d", "e", "f", "g"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_disjoint_pair_blocks() {
        let out = Solution::max_num_of_substrings("aabbccdd".to_string());
        assert_valid_cover("aabbccdd", &out);
        assert_eq!(normalize(out), vec!["aa", "bb", "cc", "dd"]);
    }

    #[test]
    fn test_interleaved_forces_whole_string() {
        // "abab": 'a' at 0,2 and 'b' at 1,3 lock each other into one window.
        let out = Solution::max_num_of_substrings("abab".to_string());
        assert_valid_cover("abab", &out);
        assert_eq!(out, vec!["abab".to_string()]);
    }

    #[test]
    fn test_fully_nested_picks_innermost() {
        // "abcba": windows are [0,4]="abcba", [1,3]="bcb", [2,2]="c".
        // All give count 1; min total length picks "c".
        let out = Solution::max_num_of_substrings("abcba".to_string());
        assert_valid_cover("abcba", &out);
        assert_eq!(out, vec!["c".to_string()]);
    }

    #[test]
    fn test_prefers_more_pieces_over_shorter_coverage() {
        // "abbaccd" has the alternative ["d","abba","cc"] with the same count
        // but larger total length — the correct answer stays ["bb","cc","d"].
        let out = Solution::max_num_of_substrings("abbaccd".to_string());
        assert_eq!(normalize(out), vec!["bb", "cc", "d"]);
    }

    #[test]
    fn test_two_disjoint_blocks_with_inner_nest() {
        // Left block "abba" contains inner "bb"; right block "ccddcc" contains
        // inner "dd". The greedy picks "bb" and "dd" instead of the outer
        // envelopes — same count (2), but minimum total length.
        let s = "abbaccddcc";
        let out = Solution::max_num_of_substrings(s.to_string());
        assert_valid_cover(s, &out);
        assert_eq!(normalize(out), vec!["bb", "dd"]);
    }

    #[test]
    fn test_fully_nested_palindrome_alphabet() {
        // In "abc…zzy…a" every letter k owns window [k, 51 - k]; those are
        // strictly nested, so count collapses to 1 and the innermost "zz" wins.
        let s = "abcdefghijklmnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcba";
        let out = Solution::max_num_of_substrings(s.to_string());
        assert_valid_cover(s, &out);
        assert_eq!(out, vec!["zz".to_string()]);
    }

    #[test]
    fn test_stress_long_abc_cycle_locks_whole_string() {
        // "abc" * 10_000 → each of a,b,c has `first` and `last` separated by
        // ~30k, so the three letters mutually trap each other into one window.
        let s = "abc".repeat(10_000);
        let out = Solution::max_num_of_substrings(s.clone());
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].len(), s.len());
    }

    #[test]
    fn test_stress_many_isolated_blocks() {
        // 26 disjoint "xx" blocks (one per letter): all 26 picked.
        let s: String = (b'a'..=b'z')
            .flat_map(|c| std::iter::repeat_n(c as char, 2))
            .collect();
        let out = Solution::max_num_of_substrings(s.clone());
        assert_valid_cover(&s, &out);
        assert_eq!(out.len(), 26);
    }
}
