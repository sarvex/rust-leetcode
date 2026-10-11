impl Solution {
    /// Minimum-removal search that stops at the first unmatched parenthesis.
    ///
    /// # Intuition
    /// A string fails when a prefix has more `)` than `(`, or the whole string
    /// still has extra `(`. Each failure needs one deletion. Trying every
    /// distinct parenthesis that could be that extra one, then repeating,
    /// lists every valid string reachable by the fewest deletions.
    ///
    /// # Approach
    /// Scan left to right with a balance count. When the balance goes
    /// negative, delete one `)` from the current segment and continue from
    /// the failure in the shortened string. Only the first `)` of a
    /// consecutive run is tried, because deleting a later twin rebuilds the
    /// same string. A scan that stays non-negative can still leave extra `(`.
    /// Reverse the string and repeat with `(` and `)` swapped, so those opens
    /// are removed by the same rule. Reverse once more before storing the result.
    /// Letters never change the balance and are never deleted.
    ///
    /// # Complexity
    /// - Time: O(n * 2^n) — each parenthesis is kept or deleted, and every
    ///   call copies or scans a string of length n. At most 20 parentheses.
    /// - Space: O(n^2) — an O(n) string on each level, with depth at most n,
    ///   excluding the output.
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn search(
            s: &str,
            start: usize,
            last_remove: usize,
            open: u8,
            close: u8,
            result: &mut Vec<String>,
        ) {
            let bytes = s.as_bytes();
            let mut balance = 0i32;
            for i in start..bytes.len() {
                if bytes[i] == open {
                    balance += 1;
                } else if bytes[i] == close {
                    balance -= 1;
                }
                if balance >= 0 {
                    continue;
                }
                for j in last_remove..=i {
                    if bytes[j] == close && (j == last_remove || bytes[j - 1] != close) {
                        let mut next = String::with_capacity(s.len() - 1);
                        next.push_str(&s[..j]);
                        next.push_str(&s[j + 1..]);
                        search(&next, i, j, open, close, result);
                    }
                }
                return;
            }

            let reversed: String = s.chars().rev().collect();
            if open == b'(' {
                search(&reversed, 0, 0, b')', b'(', result);
            } else {
                result.push(reversed);
            }
        }

        let mut result = Vec::new();
        search(&s, 0, 0, b'(', b')', &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn assert_answers(s: &str, expected: &[&str]) {
        let mut result = Solution::remove_invalid_parentheses(s.to_string());
        result.sort_unstable();
        let mut expected: Vec<String> = expected
            .iter()
            .map(|answer| (*answer).to_string())
            .collect();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    fn is_valid(s: &str) -> bool {
        let mut balance = 0i32;
        for byte in s.bytes() {
            match byte {
                b'(' => balance += 1,
                b')' => {
                    balance -= 1;
                    if balance < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        balance == 0
    }

    fn min_removals(s: &str) -> usize {
        let (mut open, mut close) = (0i32, 0i32);
        for byte in s.bytes() {
            match byte {
                b'(' => open += 1,
                b')' if open == 0 => close += 1,
                b')' => open -= 1,
                _ => {}
            }
        }
        (open + close) as usize
    }

    /// Every way to delete a subset of parentheses, keeping the smallest deletions.
    fn brute(s: &str) -> Vec<String> {
        let paren_at: Vec<usize> = s
            .bytes()
            .enumerate()
            .filter(|(_, byte)| *byte == b'(' || *byte == b')')
            .map(|(index, _)| index)
            .collect();
        let patterns = paren_at.len();
        let mut best = patterns + 1;
        let mut answers = BTreeSet::new();
        for mask in 0..(1usize << patterns) {
            let removed = mask.count_ones() as usize;
            if removed > best {
                continue;
            }
            let mut drop = vec![false; s.len()];
            for bit in 0..patterns {
                if mask & (1 << bit) != 0 {
                    drop[paren_at[bit]] = true;
                }
            }
            let kept: String = s
                .bytes()
                .enumerate()
                .filter(|(index, _)| !drop[*index])
                .map(|(_, byte)| char::from(byte))
                .collect();
            if !is_valid(&kept) {
                continue;
            }
            if removed < best {
                best = removed;
                answers.clear();
            }
            answers.insert(kept);
        }
        answers.into_iter().collect()
    }

    #[test]
    fn example_two_removals_of_one_close() {
        assert_answers("()())()", &["(())()", "()()()"]);
    }

    #[test]
    fn example_letter_inside() {
        assert_answers("(a)())()", &["(a())()", "(a)()()"]);
    }

    #[test]
    fn example_both_parentheses_removed() {
        assert_answers(")(", &[""]);
    }

    #[test]
    fn single_open() {
        assert_answers("(", &[""]);
    }

    #[test]
    fn single_close() {
        assert_answers(")", &[""]);
    }

    #[test]
    fn already_valid() {
        assert_answers("(())", &["(())"]);
        assert_answers("()()()", &["()()()"]);
        assert_answers("(a)", &["(a)"]);
    }

    #[test]
    fn letters_only() {
        assert_answers("abc", &["abc"]);
        assert_answers("n", &["n"]);
    }

    #[test]
    fn letter_between_mismatched_pair() {
        assert_answers(")a(", &["a"]);
    }

    #[test]
    fn extra_open_or_close() {
        assert_answers("(()", &["()"]);
        assert_answers("())", &["()"]);
        assert_answers("((()", &["()"]);
        assert_answers("()(()", &["()()"]);
    }

    #[test]
    fn several_minimum_strings() {
        assert_answers("()())", &["(())", "()()"]);
        assert_answers("(())())", &["(())()", "(()())"]);
        assert_answers("(((k()((", &["(k)", "k()"]);
    }

    #[test]
    fn all_opens_or_closes() {
        assert_answers("(((", &[""]);
        assert_answers(")))", &[""]);
    }

    #[test]
    fn matches_brute_force_on_generated_strings() {
        let mut state = 0x1234_5678u32;
        let alphabet = b"()abc";
        for len in 0..=10 {
            for _ in 0..20 {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let sample: String = (0..len)
                    .map(|offset| {
                        let mixed = state.wrapping_add(offset as u32);
                        char::from(alphabet[(mixed as usize) % alphabet.len()])
                    })
                    .collect();
                let mut actual = Solution::remove_invalid_parentheses(sample.clone());
                actual.sort_unstable();
                assert_eq!(actual, brute(&sample), "mismatch for {sample:?}");
            }
        }
    }

    #[test]
    fn longer_input_is_valid_and_minimum() {
        let sample = "()())())())())())()";
        let result = Solution::remove_invalid_parentheses(sample.to_string());
        let target_len = sample.len() - min_removals(sample);
        let unique: BTreeSet<_> = result.iter().cloned().collect();
        assert_eq!(unique.len(), result.len());
        assert!(result.len() > 1);
        assert!(
            result
                .iter()
                .all(|answer| answer.len() == target_len && is_valid(answer))
        );
    }
}
