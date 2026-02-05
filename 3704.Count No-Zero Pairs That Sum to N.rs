impl Solution {
    /// Counts pairs (a, b) where a + b = n and both are no-zero integers.
    ///
    /// # Intuition
    /// Model each number as "active" or "ended" (all higher digits are 0).
    /// Once a number picks 0, it ends and must stay 0 - preventing interior zeros.
    ///
    /// # Approach
    /// Iterative digit DP from LSB to MSB. At position 0, both must pick 1-9.
    /// State: (carry, a_ended, b_ended). Only 8 states total.
    ///
    /// # Complexity
    /// - Time: O(d) where d ≤ 16 digits
    /// - Space: O(1)
    pub fn count_no_zero_pairs(n: i64) -> i64 {
        let mut target_digits = [0u8; 16];
        let num_digits = std::iter::successors(Some(n), |&x| (x >= 10).then(|| x / 10))
            .enumerate()
            .map(|(i, x)| {
                target_digits[i] = (x % 10) as u8;
                i + 1
            })
            .last()
            .unwrap_or(1);

        // dp[carry][a_ended][b_ended] = count
        let mut dp = [[[0i64; 2]; 2]; 2];

        // Position 0: both must pick 1-9 (positive), neither ended
        let target = target_digits[0] as i32;
        (1..=9)
            .flat_map(|a| (1..=9).map(move |b| a + b))
            .filter(|&sum| sum % 10 == target)
            .for_each(|sum| dp[(sum / 10) as usize][0][0] += 1);

        // Process remaining positions
        for pos in 1..num_digits {
            let target = target_digits[pos] as i32;
            let mut next_dp = [[[0i64; 2]; 2]; 2];

            for carry_in in 0..2i32 {
                for a_ended in 0..2usize {
                    for b_ended in 0..2usize {
                        let count = dp[carry_in as usize][a_ended][b_ended];
                        if count == 0 {
                            continue;
                        }

                        // Determine valid digit ranges
                        let a_range = if a_ended == 1 { 0..=0 } else { 0..=9 };
                        let b_range = if b_ended == 1 { 0..=0 } else { 0..=9 };

                        for digit_a in a_range {
                            for digit_b in b_range.clone() {
                                let sum = digit_a + digit_b + carry_in;
                                if sum % 10 == target {
                                    let next_a = if a_ended == 1 || digit_a == 0 { 1 } else { 0 };
                                    let next_b = if b_ended == 1 || digit_b == 0 { 1 } else { 0 };
                                    next_dp[(sum / 10) as usize][next_a][next_b] += count;
                                }
                            }
                        }
                    }
                }
            }

            dp = next_dp;
        }

        // Sum states with no remaining carry
        dp[0].iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_no_zero_pairs(2), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_no_zero_pairs(3), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_no_zero_pairs(11), 8);
    }

    #[test]
    fn test_n_10() {
        assert_eq!(Solution::count_no_zero_pairs(10), 9);
    }

    #[test]
    fn test_n_20() {
        assert_eq!(Solution::count_no_zero_pairs(20), 18);
    }

    #[test]
    fn test_n_100() {
        assert_eq!(Solution::count_no_zero_pairs(100), 90);
    }

    #[test]
    fn test_large_n() {
        let result = Solution::count_no_zero_pairs(1_000_000_000_000_000);
        assert!(result > 0);
    }
}
