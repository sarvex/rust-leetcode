impl Solution {
    /// Count valid substrings by run-length encoding then summing min of adjacent runs.
    ///
    /// # Intuition
    /// Valid substrings are exactly those with two consecutive groups of 0's and 1's
    /// with equal length (e.g. "01", "10", "0011", "1100"). So we run-length encode
    /// the string into group lengths, then for each adjacent pair the number of valid
    /// substrings spanning that boundary is min(prev_run, curr_run).
    ///
    /// # Approach
    /// 1. Build a list of run lengths (consecutive same character counts).
    /// 2. Sum min(groups[i], groups[i + 1]) for each adjacent pair.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for run lengths; can be O(1) with a single pass tracking only
    ///   prev and curr run length.
    pub fn count_binary_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes.len() < 2 {
            return 0;
        }
        let mut groups = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];
            let mut count = 0;
            while i < bytes.len() && bytes[i] == b {
                count += 1;
                i += 1;
            }
            groups.push(count);
        }
        groups
            .windows(2)
            .map(|w| std::cmp::min(w[0], w[1]) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::count_binary_substrings("00110011".into()), 6);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::count_binary_substrings("10101".into()), 4);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::count_binary_substrings("0".into()), 0);
        assert_eq!(Solution::count_binary_substrings("1".into()), 0);
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(Solution::count_binary_substrings("01".into()), 1);
        assert_eq!(Solution::count_binary_substrings("10".into()), 1);
        assert_eq!(Solution::count_binary_substrings("00".into()), 0);
    }
}
