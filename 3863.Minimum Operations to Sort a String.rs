impl Solution {
    /// Minimum "sort a proper substring" operations to fully sort `s`.
    ///
    /// # Intuition
    /// The sorted target just places the smallest byte first and the largest
    /// byte last. Each operation sorts any substring except the whole string,
    /// so the answer collapses to a small set of cases driven by the string
    /// length and the two endpoints:
    /// - Length 1: a single character is already sorted.
    /// - Length 2: every proper substring is one character, so sorting is a
    ///   no-op; the string is either already sorted or forever unsortable.
    /// - Length >= 3: a proper substring of length `n - 1` always exists.
    ///   - If an endpoint already holds its final character, the remaining
    ///     `n - 1` block is a proper substring that can be sorted in one move.
    ///   - If both endpoints are wrong, two moves usually suffice (fix one end
    ///     with a proper prefix/suffix, then sort the rest) -- *unless* the
    ///     front holds the unique largest character and the back holds the
    ///     unique smallest. Then every proper substring that touches an endpoint
    ///     still leaves that endpoint wrong (the extreme element must travel the
    ///     whole way across), forcing a third operation. The duplicate case has
    ///     slack: a second copy of the min or max lets one move settle an end.
    ///
    /// # Approach
    /// Avoid sorting entirely. A single linear scan tells us everything the
    /// case analysis needs: whether the string is already non-descending
    /// (no adjacent inversion), the smallest and largest bytes present, and how
    /// many times each extreme occurs. Zero moves if already sorted; `-1` for
    /// an unsorted length <= 2; `1` when an endpoint already matches its
    /// extreme; `3` when the first byte is the unique maximum and the last is
    /// the unique minimum; otherwise `2`.
    ///
    /// # Complexity
    /// - Time: O(n) single pass, no sorting
    /// - Space: O(1)
    pub fn min_operations(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut min_char = u8::MAX;
        let mut max_char = u8::MIN;
        let mut min_count = 0u32;
        let mut max_count = 0u32;
        let mut is_sorted = true;

        for i in 0..n {
            let c = bytes[i];

            if i > 0 && bytes[i - 1] > c {
                is_sorted = false;
            }

            // Track the minimum byte and how many times it appears.
            if c < min_char {
                min_char = c;
                min_count = 1;
            } else if c == min_char {
                min_count += 1;
            }

            // Track the maximum byte and how many times it appears.
            if c > max_char {
                max_char = c;
                max_count = 1;
            } else if c == max_char {
                max_count += 1;
            }
        }

        if is_sorted {
            return 0;
        }

        // Not sorted from here on.
        if n <= 2 {
            return -1;
        }

        // Either endpoint already correct -> a single proper-substring sort.
        if bytes[0] == min_char || bytes[n - 1] == max_char {
            return 1;
        }

        // Both ends wrong. The expensive configuration is the front carrying the
        // unique maximum while the back carries the unique minimum.
        if bytes[0] == max_char && bytes[n - 1] == min_char && min_count == 1 && max_count == 1 {
            3
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_sort_once() {
        assert_eq!(Solution::min_operations("dog".to_string()), 1);
    }

    #[test]
    fn both_ends_wrong_needs_two() {
        assert_eq!(Solution::min_operations("card".to_string()), 2);
    }

    #[test]
    fn length_two_unsortable() {
        assert_eq!(Solution::min_operations("gf".to_string()), -1);
    }

    #[test]
    fn reversed_unique_extremes_needs_three() {
        assert_eq!(Solution::min_operations("edc".to_string()), 3);
    }

    #[test]
    fn fully_reversed_needs_three() {
        assert_eq!(Solution::min_operations("cba".to_string()), 3);
    }

    #[test]
    fn already_sorted() {
        assert_eq!(Solution::min_operations("abc".to_string()), 0);
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::min_operations("a".to_string()), 0);
    }

    #[test]
    fn length_two_sorted() {
        assert_eq!(Solution::min_operations("fg".to_string()), 0);
    }

    #[test]
    fn first_char_correct() {
        // 'a' already leads; sort "cb" -> "bc" in one move.
        assert_eq!(Solution::min_operations("acb".to_string()), 1);
    }

    #[test]
    fn last_char_correct() {
        // 'c' already trails; sort "ba" -> "ab" in one move.
        assert_eq!(Solution::min_operations("bac".to_string()), 1);
    }

    #[test]
    fn duplicate_extreme_keeps_two() {
        // Front is max, back is min, but the min 'a' repeats -> only 2.
        assert_eq!(Solution::min_operations("baa".to_string()), 2);
    }

    #[test]
    fn duplicate_max_keeps_two() {
        // Front is max, back is min, but the max 'b' repeats -> only 2.
        assert_eq!(Solution::min_operations("bba".to_string()), 2);
    }
}
