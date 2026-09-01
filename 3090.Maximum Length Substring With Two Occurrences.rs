impl Solution {
    /// Sliding window over the string tracking per-character counts.
    ///
    /// # Intuition
    /// A valid substring allows each character to appear at most twice. As we
    /// expand a window to the right, the only way it becomes invalid is when the
    /// newly added character reaches a third occurrence. We then shrink from the
    /// left until that character drops back to two, keeping the window valid.
    ///
    /// # Approach
    /// Maintain a fixed-size count array of 26 lowercase letters and two pointers
    /// `left`/`right`. For each character entering on the right, increment its
    /// count; while that count exceeds 2, decrement the leftmost character and
    /// advance `left`. Track the maximum window width seen along the way.
    ///
    /// # Complexity
    /// - Time: O(n) — each index enters and leaves the window at most once.
    /// - Space: O(1) — a fixed 26-element count array.
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut counts = [0u8; 26];
        let mut left = 0usize;
        let mut best = 0usize;

        for (right, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            counts[idx] += 1;
            while counts[idx] > 2 {
                counts[(bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }
            best = best.max(right - left + 1);
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(
            Solution::maximum_length_substring("bcbbbcba".to_string()),
            4
        );
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }

    #[test]
    fn test_min_length_no_repeat() {
        assert_eq!(Solution::maximum_length_substring("ab".to_string()), 2);
    }

    #[test]
    fn test_min_length_same_char() {
        assert_eq!(Solution::maximum_length_substring("aa".to_string()), 2);
    }

    #[test]
    fn test_all_distinct() {
        assert_eq!(Solution::maximum_length_substring("abcdef".to_string()), 6);
    }

    #[test]
    fn test_triple_in_middle() {
        // "aaa" forces a shrink; best valid window keeps two 'a's plus surroundings.
        assert_eq!(Solution::maximum_length_substring("baaab".to_string()), 3);
    }
}
