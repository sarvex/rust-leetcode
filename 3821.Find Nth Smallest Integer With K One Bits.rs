use std::sync::OnceLock;

const MAX_N: usize = 50;

static BINOMIAL: OnceLock<Vec<Vec<u64>>> = OnceLock::new();

fn get_binomial() -> &'static Vec<Vec<u64>> {
    BINOMIAL.get_or_init(|| {
        let mut binomial = vec![vec![0u64; MAX_N + 1]; MAX_N + 1];
        (0..=MAX_N).for_each(|i| {
            binomial[i][0] = 1;
            (1..=i).for_each(|j| {
                binomial[i][j] = binomial[i - 1][j - 1] + binomial[i - 1][j];
            });
        });
        binomial
    })
}

impl Solution {
    /// Greedy bit construction using combinatorics to find nth smallest number with k ones.
    ///
    /// # Intuition
    /// Numbers with exactly k ones are combinations of k bit positions. We can greedily
    /// determine each bit position by counting how many valid numbers exist if we don't set
    /// the current bit, using binomial coefficients C(remaining_bits, remaining_ones).
    ///
    /// # Approach
    /// Precompute binomial coefficients C(m, k) once using lazy static initialization.
    /// For each bit position from MSB to LSB, count valid numbers if we skip setting this bit.
    /// If count < n, we must set the bit (reduce k and adjust n). Otherwise, skip the bit.
    ///
    /// # Complexity
    /// - Time: O(50) per query, O(50²) one-time precomputation
    /// - Space: O(50²) for static binomial coefficient table
    pub fn nth_smallest(n: i64, k: i32) -> i64 {
        let n = n as u64;
        let k = k as usize;
        let binomial = get_binomial();

        let mut result = 0u64;
        let mut remaining_ones = k;
        let mut remaining_n = n;

        // Process bits from MSB (49) down to LSB (0)
        // Since answer < 2^50, we need at most 50 bits
        for bit_pos in (0..50).rev() {
            if remaining_ones == 0 {
                break;
            }

            // Count numbers with k ones if we don't set this bit
            // We have 'bit_pos' remaining positions and 'remaining_ones' ones to place
            let count_if_skip = if bit_pos >= remaining_ones {
                binomial[bit_pos][remaining_ones]
            } else {
                0
            };

            if count_if_skip < remaining_n {
                // Must set this bit
                result |= 1u64 << bit_pos;
                remaining_n -= count_if_skip;
                remaining_ones -= 1;
            }
            // Otherwise, skip this bit (don't set it)
        }

        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::nth_smallest(4, 2), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::nth_smallest(3, 1), 4);
    }

    #[test]
    fn first_number() {
        assert_eq!(Solution::nth_smallest(1, 2), 3);
    }

    #[test]
    fn single_one() {
        assert_eq!(Solution::nth_smallest(1, 1), 1);
        assert_eq!(Solution::nth_smallest(2, 1), 2);
        assert_eq!(Solution::nth_smallest(3, 1), 4);
    }

    #[test]
    fn two_ones() {
        assert_eq!(Solution::nth_smallest(1, 2), 3); // 11
        assert_eq!(Solution::nth_smallest(2, 2), 5); // 101
        assert_eq!(Solution::nth_smallest(3, 2), 6); // 110
        assert_eq!(Solution::nth_smallest(4, 2), 9); // 1001
    }

    #[test]
    fn large_n() {
        // Test with larger n values
        let result = Solution::nth_smallest(100, 5);
        assert!(result > 0);
        assert_eq!(result.count_ones(), 5);
    }
}
