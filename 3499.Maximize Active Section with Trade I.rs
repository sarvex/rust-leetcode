pub struct Solution;

impl Solution {
    /// Single-pass scan tracking adjacent zero/one/zero run lengths to find the best trade.
    ///
    /// # Intuition
    /// Augment s with '1' at both ends. A valid trade picks a '1'-block flanked by '0'-blocks,
    /// collapses it to zeros (merging the two surrounding zero-blocks), then flips the merged
    /// block to ones. Net gain = left_zeros + right_zeros. Maximise this over all valid triplets.
    ///
    /// # Approach
    /// Scan the bytes once, maintaining:
    ///   - `baseline`  : running count of '1's (final answer base)
    ///   - `prev_zeros`: length of the zero-block immediately before the current one-block
    ///   - `ones`      : length of the current one-block
    ///   - `zeros`     : length of the current (growing) zero-block
    ///   - `max_gain`  : best prev_zeros + next_zeros seen so far
    ///
    /// Whenever a zero-block closes (we see a '1' after zeros > 0), we evaluate the triplet
    /// (prev_zeros, ones, zeros) and update max_gain.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut baseline = 0i32;
        let mut max_gain = 0i32;

        // prev_zeros: zero-run before the last ones-run
        // ones:       length of the last ones-run
        // zeros:      length of the current (possibly growing) zeros-run
        let mut prev_zeros = 0i32;
        let mut ones = 0i32;
        let mut zeros = 0i32;

        for &b in bytes {
            if b == b'1' {
                baseline += 1;
                if zeros > 0 {
                    // A zero-block just closed. Evaluate triplet (prev_zeros, ones, zeros).
                    // ones > 0 means there was a real 1-block between the two zero-blocks.
                    if ones > 0 && prev_zeros > 0 {
                        max_gain = max_gain.max(prev_zeros + zeros);
                    }
                    // Slide the window: the zero-block that just closed becomes prev_zeros,
                    // the ones-block resets.
                    prev_zeros = zeros;
                    zeros = 0;
                    ones = 1;
                } else {
                    ones += 1;
                }
            } else {
                zeros += 1;
            }
        }

        // Handle the trailing zero-block (augmented '1' closes it on the right).
        // Only valid if there was a ones-block AND a prev zero-block before it.
        if zeros > 0 && ones > 0 && prev_zeros > 0 {
            max_gain = max_gain.max(prev_zeros + zeros);
        }

        baseline + max_gain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "01" — no '1'-block surrounded by '0's, no valid trade
        assert_eq!(
            Solution::max_active_sections_after_trade("01".to_string()),
            1
        );
    }

    #[test]
    fn test_example_2() {
        // "0100" → baseline=1, best gain=1+2=3, answer=4
        assert_eq!(
            Solution::max_active_sections_after_trade("0100".to_string()),
            4
        );
    }

    #[test]
    fn test_example_3() {
        // "1000100" → baseline=2, best gain=3+2=5, answer=7
        assert_eq!(
            Solution::max_active_sections_after_trade("1000100".to_string()),
            7
        );
    }

    #[test]
    fn test_example_4() {
        // "01010" → baseline=2, best gain=1+1=2, answer=4
        assert_eq!(
            Solution::max_active_sections_after_trade("01010".to_string()),
            4
        );
    }

    #[test]
    fn test_all_ones() {
        // No '0'-blocks — no trade possible
        assert_eq!(
            Solution::max_active_sections_after_trade("111".to_string()),
            3
        );
    }

    #[test]
    fn test_all_zeros() {
        // Single '0'-block, never flanked by two zero-blocks
        assert_eq!(
            Solution::max_active_sections_after_trade("000".to_string()),
            0
        );
    }

    #[test]
    fn test_single_char_one() {
        assert_eq!(
            Solution::max_active_sections_after_trade("1".to_string()),
            1
        );
    }

    #[test]
    fn test_single_char_zero() {
        assert_eq!(
            Solution::max_active_sections_after_trade("0".to_string()),
            0
        );
    }

    #[test]
    fn test_multiple_trades_pick_best() {
        // "00100010": triplet1 zeros=2,ones=1,zeros=3 → gain=5; triplet2 zeros=3,ones=1,zeros=2 → gain=5
        // baseline=2, answer=7
        assert_eq!(
            Solution::max_active_sections_after_trade("00100010".to_string()),
            7
        );
    }

    #[test]
    fn test_trailing_zero_no_left_flank() {
        // "10" — the '1' has no left zero-block, so no valid trade
        assert_eq!(
            Solution::max_active_sections_after_trade("10".to_string()),
            1
        );
    }

    #[test]
    fn test_asymmetric_pick_larger() {
        // "000100" → zeros=3,ones=1,zeros=2 → gain=5, baseline=1, answer=6
        assert_eq!(
            Solution::max_active_sections_after_trade("000100".to_string()),
            6
        );
    }
}
