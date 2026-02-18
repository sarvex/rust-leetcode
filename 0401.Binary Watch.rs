impl Solution {
    /// Enumerates valid binary watch times by bucketing hours and minutes by popcount.
    ///
    /// # Intuition
    /// A binary watch has 4 hour LEDs and 6 minute LEDs. Only (h, m) pairs
    /// whose combined set-bit count equals `turned_on` are valid. Bucket
    /// hours by popcount (0..=4) and minutes by popcount (0..=6), then
    /// enumerate pairs (h_bits, m_bits) with h_bits + m_bits == turned_on.
    ///
    /// # Approach
    /// 1. Bucket hours 0–11 by popcount (5 buckets) and minutes 0–59 by popcount (7 buckets).
    /// 2. For each h_bits in 0..=4, set m_bits = turned_on - h_bits; if m_bits in 0..=6,
    ///    take the Cartesian product of the two buckets and format each (h, m).
    /// 3. No filtering: we only allocate and format for valid pairs.
    ///
    /// # Complexity
    /// - Time: O(1) — 12 + 60 bucketing steps plus output size (at most ~150 strings)
    /// - Space: O(1) excluding the result
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let target = turned_on;
        let mut hours_by_bits: [Vec<u32>; 5] = Default::default();
        for h in 0u32..12 {
            let b = h.count_ones() as usize;
            hours_by_bits[b].push(h);
        }
        let mut minutes_by_bits: [Vec<u32>; 7] = Default::default();
        for m in 0u32..60 {
            let b = m.count_ones() as usize;
            minutes_by_bits[b].push(m);
        }
        let mut result = Vec::new();
        for h_bits in 0..=4usize {
            let m_bits_i = target - h_bits as i32;
            if (0..=6).contains(&m_bits_i) {
                let m_bits = m_bits_i as usize;
                for &h in &hours_by_bits[h_bits] {
                    for &m in &minutes_by_bits[m_bits] {
                        result.push(format!("{h}:{m:02}"));
                    }
                }
            }
        }
        result
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
