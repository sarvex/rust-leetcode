impl Solution {
    /// Counts complete substrings using segmented sliding windows.
    ///
    /// # Intuition
    /// Any valid substring cannot cross a pair of adjacent letters with alphabet distance greater
    /// than 2, so we can split `word` into maximal valid segments. Inside a segment, a complete
    /// substring with `m` distinct letters must have length `m * k`, with each present letter
    /// appearing exactly `k` times.
    ///
    /// # Approach
    /// - Split the string into maximal segments where adjacent letters differ by at most 2.
    /// - For each segment and each possible distinct count `m`:
    ///   - Slide a fixed window of length `m * k`.
    ///   - Track frequencies, how many letters are present, and how many are exactly `k`.
    ///   - Count windows where both the present count and exact-`k` count equal `m`.
    ///
    /// # Complexity
    /// - Time: O(26 * n), where `n = word.len()`
    /// - Space: O(1)
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let mut letters = word.into_bytes();
        for value in &mut letters {
            *value -= b'a';
        }
        let k = k as usize;
        if letters.is_empty() || k == 0 {
            return 0;
        }

        let mut total = 0_i64;
        let mut start = 0_usize;

        for i in 1..=letters.len() {
            let is_break = if i == letters.len() {
                true
            } else {
                letters[i].abs_diff(letters[i - 1]) > 2
            };
            if is_break {
                total += Self::count_segment(&letters[start..i], k) as i64;
                start = i;
            }
        }

        total as i32
    }

    fn count_segment(segment: &[u8], k: usize) -> i32 {
        let len = segment.len();
        if len < k {
            return 0;
        }

        let mut mask = 0_u32;
        for &value in segment {
            mask |= 1_u32 << (value as u32);
        }
        let unique = mask.count_ones() as usize;
        let max_distinct = (len / k).min(unique).min(26);
        if max_distinct == 0 {
            return 0;
        }

        let mut total = 0_i32;

        for distinct in 1..=max_distinct {
            let window_len = distinct * k;
            let mut freq = [0_usize; 26];
            let mut non_zero = 0_usize;
            let mut exact_k = 0_usize;
            let mut start = 0_usize;

            for end in 0..len {
                let idx = segment[end] as usize;
                let prev = freq[idx];
                if prev == 0 {
                    non_zero += 1;
                }
                if prev == k {
                    exact_k -= 1;
                }
                freq[idx] = prev + 1;
                if freq[idx] == k {
                    exact_k += 1;
                }

                if end >= window_len {
                    let out_idx = segment[start] as usize;
                    let prev_out = freq[out_idx];
                    if prev_out == k {
                        exact_k -= 1;
                    }
                    if prev_out == 1 {
                        non_zero -= 1;
                    }
                    freq[out_idx] = prev_out - 1;
                    if freq[out_idx] == k {
                        exact_k += 1;
                    }
                    start += 1;
                }

                if end + 1 >= window_len && exact_k == distinct && non_zero == distinct {
                    total += 1;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let word = "igigee".to_string();
        let k = 2;
        assert_eq!(Solution::count_complete_substrings(word, k), 3);
    }

    #[test]
    fn test_example_2() {
        let word = "aaabbbccc".to_string();
        let k = 3;
        assert_eq!(Solution::count_complete_substrings(word, k), 6);
    }

    #[test]
    fn test_single_char() {
        let word = "a".to_string();
        let k = 1;
        assert_eq!(Solution::count_complete_substrings(word, k), 1);
    }

    #[test]
    fn test_break_on_large_gap() {
        let word = "az".to_string();
        let k = 1;
        assert_eq!(Solution::count_complete_substrings(word, k), 2);
    }

    #[test]
    fn test_no_valid_substrings() {
        let word = "abc".to_string();
        let k = 2;
        assert_eq!(Solution::count_complete_substrings(word, k), 0);
    }
}
