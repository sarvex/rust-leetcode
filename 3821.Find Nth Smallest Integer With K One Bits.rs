use std::sync::OnceLock;

const MAX_N: usize = 50;

static BINOMIAL: OnceLock<[[u64; MAX_N + 1]; MAX_N + 1]> = OnceLock::new();

fn get_binomial() -> &'static [[u64; MAX_N + 1]; MAX_N + 1] {
    BINOMIAL.get_or_init(|| {
        let mut binomial = [[0u64; MAX_N + 1]; MAX_N + 1];
        for i in 0..=MAX_N {
            binomial[i][0] = 1;
            for j in 1..=i {
                binomial[i][j] = binomial[i - 1][j - 1] + binomial[i - 1][j];
            }
        }
        binomial
    })
}

impl Solution {
    /// Greedy bit construction using combinatorics to find nth smallest k-ones number
    ///
    /// # Intuition
    /// Numbers with exactly k ones are combinations of k bit positions. Greedily
    /// determine each bit position by counting how many valid numbers exist if we
    /// skip the current bit, using binomial coefficients C(remaining_bits, remaining_ones).
    ///
    /// # Approach
    /// Precompute binomial coefficients C(m, k) once via lazy static initialization.
    /// For each bit position from MSB to LSB, count valid numbers if we skip setting
    /// this bit. If count < n, we must set the bit and adjust remaining counts.
    ///
    /// # Complexity
    /// - Time: O(50) per query, O(50^2) one-time precomputation
    /// - Space: O(50^2) for static binomial coefficient table
    pub fn nth_smallest(n: i64, k: i32) -> i64 {
        let n = n as u64;
        let k = k as usize;
        let binomial = get_binomial();

        let mut result = 0u64;
        let mut remaining_ones = k;
        let mut remaining_n = n;

        for bit_pos in (0..MAX_N).rev() {
            if remaining_ones == 0 {
                break;
            }

            let count_if_skip = if bit_pos >= remaining_ones {
                binomial[bit_pos][remaining_ones]
            } else {
                0
            };

            if count_if_skip < remaining_n {
                result |= 1u64 << bit_pos;
                remaining_n -= count_if_skip;
                remaining_ones -= 1;
            }
        }

        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fourth_with_two_ones() {
        assert_eq!(Solution::nth_smallest(4, 2), 9);
    }

    #[test]
    fn test_third_with_one_bit() {
        assert_eq!(Solution::nth_smallest(3, 1), 4);
    }

    #[test]
    fn test_first_with_two_ones() {
        assert_eq!(Solution::nth_smallest(1, 2), 3);
    }

    #[test]
    fn test_single_one_bit_sequence() {
        assert_eq!(Solution::nth_smallest(1, 1), 1);
        assert_eq!(Solution::nth_smallest(2, 1), 2);
        assert_eq!(Solution::nth_smallest(3, 1), 4);
    }

    #[test]
    fn test_two_ones_sequence() {
        assert_eq!(Solution::nth_smallest(1, 2), 3);
        assert_eq!(Solution::nth_smallest(2, 2), 5);
        assert_eq!(Solution::nth_smallest(3, 2), 6);
        assert_eq!(Solution::nth_smallest(4, 2), 9);
    }

    #[test]
    fn test_large_n_has_correct_popcount() {
        let result = Solution::nth_smallest(100, 5);
        assert!(result > 0);
        assert_eq!(result.count_ones(), 5);
    }
}
