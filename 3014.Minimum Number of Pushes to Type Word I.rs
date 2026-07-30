impl Solution {
    /// Closed-form O(1) formula for minimum pushes with distinct letters on 8 keys.
    ///
    /// # Intuition
    /// With 8 keys (2–9) and all letters distinct, the optimal assignment fills depth 1
    /// first (8 letters × 1 push), then depth 2 (8 letters × 2 pushes), and so on.
    /// This yields a closed-form sum rather than a loop.
    ///
    /// # Approach
    /// Let `n = word.len()`, `full = n / 8`, `rem = n % 8`.
    /// - `full` complete levels contribute `8 * full * (full + 1) / 2`
    /// - `rem` leftover letters at depth `full + 1` contribute `rem * (full + 1)`
    /// Total = `(full * (full + 1) / 2) * 8 + rem * (full + 1)`
    ///       = `(full + 1) * (4 * full + rem)`
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let full = n / 8;
        let rem = n % 8;
        (full + 1) * (4 * full + rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_letters() {
        // Each of the 5 letters maps to its own key at depth 1
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn test_ten_letters() {
        // First 8 letters cost 1 each (8), next 2 cost 2 each (4) => 12
        assert_eq!(Solution::minimum_pushes("xycdefghij".to_string()), 12);
    }

    #[test]
    fn test_single_letter() {
        assert_eq!(Solution::minimum_pushes("a".to_string()), 1);
    }

    #[test]
    fn test_exactly_eight_letters() {
        // All 8 fit at depth 1
        assert_eq!(Solution::minimum_pushes("abcdefgh".to_string()), 8);
    }

    #[test]
    fn test_nine_letters() {
        // 8 at depth 1 (8) + 1 at depth 2 (2) = 10
        assert_eq!(Solution::minimum_pushes("abcdefghi".to_string()), 10);
    }

    #[test]
    fn test_all_26_letters() {
        // depth 1: i=0..7  => 8 * 1 =  8
        // depth 2: i=8..15 => 8 * 2 = 16
        // depth 3: i=16..23 => 8 * 3 = 24
        // depth 4: i=24..25 => 2 * 4 =  8
        // total = 56
        assert_eq!(
            Solution::minimum_pushes("abcdefghijklmnopqrstuvwxyz".to_string()),
            56
        );
    }
}
