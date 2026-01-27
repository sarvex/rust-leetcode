impl Solution {
    /// Enumerates valid binary watch times by counting set bits.
    ///
    /// # Intuition
    /// A binary watch has 4 hour LEDs and 6 minute LEDs. Instead of
    /// backtracking through bit positions, iterate all possible (h, m) pairs
    /// and check if their combined popcount equals `turned_on`.
    ///
    /// # Approach
    /// 1. Iterate hours 0–11 and minutes 0–59.
    /// 2. Count total set bits in (h, m).
    /// 3. Collect formatted times where the count matches.
    ///
    /// # Complexity
    /// - Time: O(1) — at most 12 × 60 = 720 iterations
    /// - Space: O(1) excluding the result
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let target = turned_on as u32;
        (0u32..12)
            .flat_map(|h| {
                (0u32..60).filter_map(move |m| {
                    if (h.count_ones() + m.count_ones()) == target {
                        Some(format!("{h}:{m:02}"))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_led() {
        let mut result = Solution::read_binary_watch(1);
        result.sort();
        let mut expected = vec![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_leds() {
        assert_eq!(Solution::read_binary_watch(0), vec!["0:00"]);
    }

    #[test]
    fn test_excessive_leds() {
        assert!(Solution::read_binary_watch(9).is_empty());
    }
}
