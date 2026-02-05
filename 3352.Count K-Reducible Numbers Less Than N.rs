impl Solution {
    /// Counts k-reducible numbers less than n using digit DP on the binary string.
    ///
    /// # Intuition
    /// A number is k-reducible if repeated popcount operations reach 1 within k steps.
    /// The depth of a number equals 1 + depth(popcount(x)) for x > 1. We count
    /// numbers < n whose popcount has depth ≤ k-1.
    ///
    /// # Approach
    /// 1. Precompute depth for every possible popcount up to bit-length of n.
    /// 2. Precompute binomial coefficients modulo 10^9+7.
    /// 3. For each target popcount with acceptable depth, count binary numbers < n
    ///    having exactly that many 1-bits via combinatorial digit DP.
    ///
    /// # Complexity
    /// - Time: O(len² + len × max_popcount) where len is the binary string length
    /// - Space: O(len²) for binomial coefficients
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let k = k as usize;
        let bits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        let len = bits.len();

        if len == 1 && bits[0] == 1 {
            return 0;
        }

        let max_popcount = len.min(800);
        let depth_map: Vec<usize> = (0..=max_popcount)
            .map(|p| match p {
                0 | 1 => 0,
                _ => Self::compute_depth(p),
            })
            .collect();

        let binomial = Self::precompute_binomial(len);

        (1..=max_popcount)
            .filter(|&p| match p {
                1 => true,
                _ => depth_map[p] <= k - 1,
            })
            .map(|p| Self::count_with_popcount_less_than(&bits, p, &binomial))
            .fold(0i64, |acc, c| (acc + c) % MOD) as i32
    }

    fn compute_depth(mut x: usize) -> usize {
        let mut depth = 0;
        while x != 1 {
            x = x.count_ones() as usize;
            depth += 1;
        }
        depth
    }

    fn count_with_popcount_less_than(bits: &[u8], target: usize, binomial: &[Vec<i64>]) -> i64 {
        const MOD: i64 = 1_000_000_007;
        if target == 0 {
            return 0;
        }

        let len = bits.len();

        let result = (1..len)
            .filter(|&num_bits| target <= num_bits)
            .fold(0i64, |acc, num_bits| {
                (acc + binomial[num_bits - 1][target - 1]) % MOD
            });

        bits.iter()
            .enumerate()
            .skip(1)
            .fold((result, 1usize), |(mut acc, ones_placed), (pos, &bit)| {
                if bit == 1 {
                    let remaining_bits = len - pos - 1;
                    if ones_placed <= target {
                        let ones_needed = target - ones_placed;
                        if ones_needed <= remaining_bits {
                            acc = (acc + binomial[remaining_bits][ones_needed]) % MOD;
                        }
                    }
                    (acc, ones_placed + 1)
                } else {
                    (acc, ones_placed)
                }
            })
            .0
    }

    fn precompute_binomial(max_n: usize) -> Vec<Vec<i64>> {
        const MOD: i64 = 1_000_000_007;
        (0..=max_n).fold(vec![vec![0i64; max_n + 1]; max_n + 1], |mut c, n| {
            c[n][0] = 1;
            (1..=n).for_each(|k| {
                c[n][k] = (c[n - 1][k - 1] + c[n - 1][k]) % MOD;
            });
            c
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_111_with_k1_gives_three() {
        assert_eq!(Solution::count_k_reducible_numbers("111".to_string(), 1), 3);
    }

    #[test]
    fn binary_1000_with_k2_gives_six() {
        assert_eq!(
            Solution::count_k_reducible_numbers("1000".to_string(), 2),
            6
        );
    }

    #[test]
    fn binary_1_with_any_k_gives_zero() {
        assert_eq!(Solution::count_k_reducible_numbers("1".to_string(), 3), 0);
    }

    #[test]
    fn binary_100_with_k1_gives_two() {
        assert_eq!(Solution::count_k_reducible_numbers("100".to_string(), 1), 2);
    }
}
