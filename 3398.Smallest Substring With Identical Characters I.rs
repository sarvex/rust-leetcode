impl Solution {
    /// Binary search on answer with greedy validation
    ///
    /// # Intuition
    /// The minimum achievable max-length is monotonic: if we can achieve length k,
    /// we can also achieve any length > k. This suggests binary search.
    ///
    /// # Approach
    /// Binary search on the answer. For each candidate length `mid`:
    /// - If `mid == 1`: we need an alternating pattern "010101..." or "101010..."
    ///   Count mismatches for both patterns and take minimum.
    /// - If `mid > 1`: for each run of identical characters of length `len`,
    ///   we need `len / (mid + 1)` flips to break it into segments of at most `mid`.
    ///
    /// # Complexity
    /// - Time: O(n log n) - binary search with O(n) validation
    /// - Space: O(n)
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
        if max_len == 1 {
            // Need alternating pattern "010101..." or "101010..."
            let ops0: usize = chars
                .iter()
                .enumerate()
                .filter(|(i, c)| (*c - b'0') as usize != i % 2)
                .count();
            let ops1 = chars.len() - ops0;
            ops0.min(ops1) <= num_ops
        } else {
            // Count flips needed to break each run of identical characters
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "000001" with 1 op -> "001001" -> max run = 2
        assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
    }

    #[test]
    fn test_example_2() {
        // "0000" with 2 ops -> "1010" -> max run = 1
        assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
    }

    #[test]
    fn test_example_3() {
        // "0101" already alternating -> max run = 1
        assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::min_length("0".to_string(), 0), 1);
        assert_eq!(Solution::min_length("1".to_string(), 0), 1);
    }

    #[test]
    fn test_no_ops_needed() {
        assert_eq!(Solution::min_length("10101010".to_string(), 0), 1);
    }

    #[test]
    fn test_long_run() {
        // "11111" with 0 ops -> max run = 5
        assert_eq!(Solution::min_length("11111".to_string(), 0), 5);
        // "11111" with 1 op -> can break into max run = 2
        assert_eq!(Solution::min_length("11111".to_string(), 1), 2);
    }
}
