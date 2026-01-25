impl Solution {
    /// Counts integers in [1, n] with popcount-depth exactly k using digit DP.
    ///
    /// # Intuition
    /// Popcount-depth measures steps of repeated popcount until reaching 1.
    /// For x > 1: depth(x) = 1 + depth(popcount(x)). For x = 1: depth(1) = 0.
    /// To find numbers with depth k, count numbers whose popcount has depth k-1.
    ///
    /// # Approach
    /// 1. For k=0, only x=1 qualifies (depth 0 by definition)
    /// 2. For k>=1, find popcounts p with depth(p) = k-1
    /// 3. Count numbers in [1,n] with each target popcount using digit DP
    /// 4. Exclude x=1 when k=1 (special case: popcount(1)=1 but depth(1)=0)
    ///
    /// # Complexity
    /// - Time: O(64² + 64 × log(n))
    /// - Space: O(64²) for binomial coefficients
    pub fn popcount_depth(n: i64, k: i32) -> i64 {
        if k == 0 {
            return i64::from(n >= 1);
        }

        let n = n as u64;
        let target_depth = k - 1;
        let binomial = Self::precompute_binomial();

        let result: i64 = (1..=64)
            .filter(|&p| Self::compute_depth(p) == target_depth)
            .map(|p| Self::count_with_popcount(n, p, &binomial))
            .sum();

        if k == 1 { result - 1 } else { result }
    }

    fn compute_depth(mut x: usize) -> i32 {
        let mut depth = 0;
        while x != 1 {
            x = x.count_ones() as usize;
            depth += 1;
        }
        depth
    }

    fn count_with_popcount(n: u64, target: usize, binomial: &[[i64; 65]; 65]) -> i64 {
        if target == 0 || n == 0 {
            return 0;
        }

        let num_bits = 64 - n.leading_zeros() as usize;
        let mut result = 0i64;
        let mut ones_placed = 0;

        (0..num_bits).rev().for_each(|bit_pos| {
            if (n >> bit_pos) & 1 == 1 {
                if ones_placed <= target {
                    let ones_needed = target - ones_placed;
                    if ones_needed <= bit_pos {
                        result += binomial[bit_pos][ones_needed];
                    }
                }
                ones_placed += 1;
            }
        });

        if n.count_ones() as usize == target {
            result += 1;
        }

        result
    }

    fn precompute_binomial() -> [[i64; 65]; 65] {
        let mut coefficients = [[0i64; 65]; 65];
        (0..65).for_each(|n| {
            coefficients[n][0] = 1;
            (1..=n).for_each(|k| {
                coefficients[n][k] = coefficients[n - 1][k - 1] + coefficients[n - 1][k];
            });
        });
        coefficients
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::popcount_depth(4, 1), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::popcount_depth(7, 2), 3);
    }

    #[test]
    fn test_depth_zero() {
        assert_eq!(Solution::popcount_depth(1, 0), 1);
        assert_eq!(Solution::popcount_depth(100, 0), 1);
    }

    #[test]
    fn test_depth_one_small() {
        assert_eq!(Solution::popcount_depth(1, 1), 0);
        assert_eq!(Solution::popcount_depth(2, 1), 1);
        assert_eq!(Solution::popcount_depth(7, 1), 2);
        assert_eq!(Solution::popcount_depth(8, 1), 3);
    }

    #[test]
    fn test_depth_three() {
        assert_eq!(Solution::popcount_depth(7, 3), 1);
    }

    #[test]
    fn test_large_n() {
        let result = Solution::popcount_depth(1_000_000_000_000_000, 3);
        assert!(result > 0);
    }
}
