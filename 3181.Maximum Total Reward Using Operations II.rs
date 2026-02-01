const BITS: usize = 64;

impl Solution {
    /// Maximum total reward by choosing rewards in increasing order so each new reward is > current sum.
    ///
    /// # Intuition
    /// We can only add a reward when it is strictly greater than the current total. So we must add
    /// rewards in ascending order. The problem reduces to choosing a subset of (deduplicated)
    /// rewards and taking them in sorted order; the constraint is that each chosen value must
    /// exceed the sum of all previously chosen values. We maximize the total sum.
    ///
    /// # Approach
    /// - Sort and deduplicate rewards (each value used at most once).
    /// - DP: `achievable[s]` = true if we can achieve total reward `s`, stored as a bitset (u64 words).
    /// - For each value `v`, copy bits from [0, v) to [v, 2v) using word-level OR with shift so we do
    ///   O(v/64) work per v instead of O(v), avoiding TLE for n, M up to 5e4.
    /// - Max sum is at most `2 * max(rewardValues) - 1`, so a buffer of `100_000` suffices.
    ///
    /// # Complexity
    /// - Time: O(n log n + n * (M / 64)) where M = max(rewardValues).
    /// - Space: O(M)
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut sorted: Vec<i32> = reward_values.into_iter().collect();
        sorted.sort_unstable();
        sorted.dedup();

        let max_val = *sorted.last().unwrap_or(&0) as usize;
        let cap = (2 * max_val).min(100_000);
        let words = (cap + BITS) / BITS;
        let mut bits = vec![0u64; words];
        bits[0] = 1;

        for &v in &sorted {
            let v = v as usize;
            if v == 0 || v >= cap {
                continue;
            }
            let shift_lo = v % BITS;
            let shift_hi = BITS - shift_lo;
            let src_end = (v + BITS - 1) / BITS;
            for sw in (0..src_end).rev() {
                let dw = sw + v / BITS;
                if dw >= words {
                    continue;
                }
                let bits_in_word = (v - sw * BITS).min(BITS);
                let mask = if bits_in_word >= BITS {
                    u64::MAX
                } else {
                    (1 << bits_in_word) - 1
                };
                let src_bits = bits[sw] & mask;
                bits[dw] |= src_bits << shift_lo;
                if shift_lo != 0 && dw + 1 < words {
                    bits[dw + 1] |= src_bits >> shift_hi;
                }
            }
        }

        for (idx, &word) in bits.iter().enumerate().rev() {
            if word != 0 {
                let base = idx * BITS;
                let bit = base + (BITS - 1 - word.leading_zeros() as usize);
                return (bit.min(cap)) as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::max_total_reward(vec![5]), 5);
    }

    #[test]
    fn test_two_ascending() {
        assert_eq!(Solution::max_total_reward(vec![1, 2]), 3);
    }

    #[test]
    fn test_two_same() {
        assert_eq!(Solution::max_total_reward(vec![3, 3]), 3);
    }

    #[test]
    fn test_cannot_add_after_large() {
        assert_eq!(Solution::max_total_reward(vec![1, 2, 4]), 7);
    }
}
