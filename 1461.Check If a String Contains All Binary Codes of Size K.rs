impl Solution {
    /// Sliding window: mark each length-k code in a bool array; true iff every code is marked.
    ///
    /// # Intuition
    /// There are 2^k distinct binary codes of length k. Build a sliding-window value over the
    /// string, mark each code in a `done` array, then check that every entry is true. No
    /// per-window branches—just shift, add, mask, and store—so the loop is very predictable.
    ///
    /// # Approach
    /// 1. If `s.len() < k`, return false.
    /// 2. Build the first (k - 1) bits of the window in `curr`.
    /// 3. For each byte from index k-1 onward: extend `curr` to k bits with the new byte, apply
    ///    mask, set `done[curr] = true`.
    /// 4. Return whether every element of `done` is true.
    ///
    /// # Complexity
    /// - Time: O(n + 2^k)
    /// - Space: O(2^k)
    ///
    /// # Panics
    /// Never panics; constraints ensure k ∈ [1, 20] and s only contains '0'/'1'.
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s = s.as_bytes();
        let ku = k as usize;
        if s.len() < ku {
            return false;
        }

        let mut curr = 0usize;
        for &c in s.iter().take(ku - 1) {
            curr = (curr << 1) + (c - b'0') as usize;
        }

        let mask = (1_usize << ku) - 1;
        let mut done = vec![false; mask + 1];
        for &c in s.iter().skip(ku - 1) {
            curr = ((curr << 1) + (c - b'0') as usize) & mask;
            done[curr] = true;
        }

        done.iter().all(|&b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::has_all_codes("00110110".to_string(), 2));
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::has_all_codes("0110".to_string(), 1));
    }

    #[test]
    fn test_example_3() {
        assert!(!Solution::has_all_codes("0110".to_string(), 2));
    }

    #[test]
    fn test_short_string() {
        assert!(!Solution::has_all_codes("01".to_string(), 3));
    }

    #[test]
    fn test_single_bit_k1() {
        assert!(!Solution::has_all_codes("0".to_string(), 1));
        assert!(!Solution::has_all_codes("1".to_string(), 1));
    }

    #[test]
    fn test_all_codes_present() {
        let s = (0..(1 << 10)).map(|i| format!("{:010b}", i)).collect::<String>();
        assert!(Solution::has_all_codes(s, 10));
    }
}
