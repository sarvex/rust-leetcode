impl Solution {
    /// Finds the lexicographically smallest permutation of `s` strictly greater than `target`.
    ///
    /// # Intuition
    /// The answer shares the longest possible prefix with `target`, then at some
    /// position places a character strictly greater than `target`'s character and
    /// fills the rest with the remaining letters in ascending order. Breaking as
    /// late as possible yields the smallest strictly-greater permutation, so we
    /// walk along `target` keeping the prefix "tight" and record the latest legal
    /// break point.
    ///
    /// # Approach
    /// 1. Count the 26 letters of `s` into a frequency array.
    /// 2. Walk positions `i = 0..n` while the prefix can stay equal to `target`:
    ///    - Break candidate: pick the smallest available char `c > target[i]`,
    ///      then append the leftover letters sorted ascending. This is the smallest
    ///      completion once we exceed `target` at this position. Record it (a later
    ///      break overwrites an earlier one because it is lexicographically smaller).
    ///    - Tight step: if `target[i]` is still available, consume it and extend the
    ///      prefix; otherwise no further tight progress is possible, so stop.
    /// 3. The last recorded break candidate is the answer; if none exists, return "".
    ///
    /// # Complexity
    /// - Time: O(n * 26) for the walk plus O(n) to assemble each candidate → O(n^2)
    /// - Space: O(n) for the constructed answer
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let n = s.len();
        let target = target.as_bytes();

        let mut count = [0usize; 26];
        for &b in s.as_bytes() {
            count[(b - b'a') as usize] += 1;
        }

        let mut prefix: Vec<u8> = Vec::with_capacity(n);
        let mut answer: Option<Vec<u8>> = None;

        for &t in target.iter().take(n) {
            let ti = (t - b'a') as usize;

            // Break candidate: smallest available character strictly greater than t.
            if let Some(bigger) = ((ti + 1)..26).find(|&c| count[c] > 0) {
                count[bigger] -= 1;

                let mut candidate = Vec::with_capacity(n);
                candidate.extend_from_slice(&prefix);
                candidate.push(b'a' + bigger as u8);
                for (c, &freq) in count.iter().enumerate() {
                    for _ in 0..freq {
                        candidate.push(b'a' + c as u8);
                    }
                }
                answer = Some(candidate);

                count[bigger] += 1;
            }

            // Tight step: keep the prefix equal to target if the letter is available.
            if count[ti] == 0 {
                break;
            }
            count[ti] -= 1;
            prefix.push(t);
        }

        match answer {
            Some(bytes) => String::from_utf8(bytes).unwrap(),
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::lex_greater_permutation("abc".to_string(), "bba".to_string()),
            "bca"
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::lex_greater_permutation("leet".to_string(), "code".to_string()),
            "eelt"
        );
    }

    #[test]
    fn test_example_3_no_answer() {
        assert_eq!(
            Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string()),
            ""
        );
    }

    #[test]
    fn test_single_char_greater() {
        assert_eq!(
            Solution::lex_greater_permutation("b".to_string(), "a".to_string()),
            "b"
        );
    }

    #[test]
    fn test_single_char_equal_no_answer() {
        assert_eq!(
            Solution::lex_greater_permutation("a".to_string(), "a".to_string()),
            ""
        );
    }

    #[test]
    fn test_break_at_first_position() {
        // Smallest permutation of "cba" strictly greater than "aaa" is "abc".
        assert_eq!(
            Solution::lex_greater_permutation("cba".to_string(), "aaa".to_string()),
            "abc"
        );
    }

    #[test]
    fn test_prefix_matches_then_bumps() {
        // Keep "aa" prefix, then bump: permutations of "aab" > "aab" -> "aba".
        assert_eq!(
            Solution::lex_greater_permutation("aab".to_string(), "aab".to_string()),
            "aba"
        );
    }

    #[test]
    fn test_all_same_letters_no_answer() {
        assert_eq!(
            Solution::lex_greater_permutation("aaa".to_string(), "aaa".to_string()),
            ""
        );
    }

    #[test]
    fn test_late_break_preferred() {
        // "abdc" is the smallest permutation of "abcd" strictly greater than "abcd".
        assert_eq!(
            Solution::lex_greater_permutation("abcd".to_string(), "abcd".to_string()),
            "abdc"
        );
    }
}
