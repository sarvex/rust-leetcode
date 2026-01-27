impl Solution {
    /// Finds minimum longest identical-char run after at most num_ops flips.
    ///
    /// # Intuition
    /// Same monotonic property as the simpler variant — binary search on the
    /// answer with an O(n) feasibility check per candidate.
    ///
    /// # Approach
    /// 1. Binary search on candidate max_len in [1, n].
    /// 2. For max_len = 1: count mismatches against both alternating patterns.
    /// 3. For max_len > 1: each run of length L costs ⌊L / (max_len+1)⌋ flips.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(n) for byte vector
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let chars: Vec<u8> = s.bytes().collect();
        let n = chars.len();

        let (mut lo, mut hi) = (1, n);

        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::achievable(&chars, mid, num_ops as usize) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i32
    }

    fn achievable(chars: &[u8], max_len: usize, num_ops: usize) -> bool {
        match max_len {
            1 => {
                let ops0: usize = chars
                    .iter()
                    .enumerate()
                    .filter(|(i, c)| (*c - b'0') as usize != i % 2)
                    .count();
                ops0.min(chars.len() - ops0) <= num_ops
            }
            _ => {
                let mut total = 0;
                let mut i = 0;
                while i < chars.len() {
                    let run_len = chars[i..].iter().take_while(|c| **c == chars[i]).count();
                    total += run_len / (max_len + 1);
                    i += run_len;
                }
                total <= num_ops
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_flip_breaks_long_run() {
        assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
    }

    #[test]
    fn two_flips_achieve_alternating() {
        assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
    }

    #[test]
    fn already_alternating_needs_zero() {
        assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
    }

    #[test]
    fn single_character_always_one() {
        assert_eq!(Solution::min_length("0".to_string(), 0), 1);
        assert_eq!(Solution::min_length("1".to_string(), 0), 1);
    }

    #[test]
    fn no_ops_preserves_full_run() {
        assert_eq!(Solution::min_length("11111".to_string(), 0), 5);
        assert_eq!(Solution::min_length("11111".to_string(), 1), 2);
    }
}
