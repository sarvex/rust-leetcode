use std::sync::LazyLock;

const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 100_000;

static PRECOMP: LazyLock<(Vec<i64>, Vec<i64>)> = LazyLock::new(|| {
    let mut factorial = vec![1_i64; MAX_N + 1];
    let mut pow2 = vec![1_i64; MAX_N + 1];
    for i in 1..=MAX_N {
        factorial[i] = factorial[i - 1] * i as i64 % MOD;
        pow2[i] = pow2[i - 1] * 2 % MOD;
    }
    (factorial, pow2)
});

impl Solution {
    /// Counts infection sequences by combining independent segments.
    ///
    /// # Intuition
    /// Initial infections split the line into independent segments. Edge segments must infect in a
    /// fixed order, while middle segments can expand from both ends with `2^(len-1)` choices.
    /// Interleaving the segment-wise sequences yields a multinomial factor.
    ///
    /// # Approach
    /// - Precompute factorials and powers of two once (up to `n <= 10^5`).
    /// - Compute the multinomial factor `total! / product(len_i!)`.
    /// - Multiply by `2^(len-1)` for each middle segment of length `len > 0`.
    ///
    /// # Complexity
    /// - Time: O(m), where `m = sick.len()`
    /// - Space: O(1) per call, with O(MAX_N) precomputation
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        let n = n as usize;
        let infected = sick.len();
        let total = n - infected;
        let (factorial, pow2) = &*PRECOMP;

        let mut numerator = factorial[total];
        let mut denominator = 1_i64;

        let left_len = sick[0] as usize;
        denominator = denominator * factorial[left_len] % MOD;
        for i in 1..infected {
            let len = (sick[i] - sick[i - 1] - 1) as usize;
            denominator = denominator * factorial[len] % MOD;
            if len > 0 {
                numerator = numerator * pow2[len - 1] % MOD;
            }
        }
        let right_len = n - 1 - sick[infected - 1] as usize;
        denominator = denominator * factorial[right_len] % MOD;

        ((numerator * Self::mod_pow(denominator, MOD - 2)) % MOD) as i32
    }

    fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut result = 1_i64;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let n = 5;
        let sick = vec![0, 4];
        assert_eq!(Solution::number_of_sequence(n, sick), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 4;
        let sick = vec![1];
        assert_eq!(Solution::number_of_sequence(n, sick), 3);
    }

    #[test]
    fn test_single_uninfected() {
        let n = 3;
        let sick = vec![0, 1];
        assert_eq!(Solution::number_of_sequence(n, sick), 1);
    }

    #[test]
    fn test_multiple_segments() {
        let n = 7;
        let sick = vec![1, 3, 6];
        assert_eq!(Solution::number_of_sequence(n, sick), 24);
    }

    #[test]
    fn test_long_middle_segment() {
        let n = 6;
        let sick = vec![0, 5];
        assert_eq!(Solution::number_of_sequence(n, sick), 8);
    }
}
