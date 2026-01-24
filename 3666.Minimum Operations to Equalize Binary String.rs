impl Solution {
    /// Minimum Operations to Equalize Binary String
    ///
    /// # Intuition
    /// Each operation flips exactly k indices. For all characters to become '1':
    /// - Each '0' position needs an odd number of flips
    /// - Each '1' position needs an even number of flips (including zero)
    ///
    /// # Approach
    /// 1. Check impossibility conditions:
    ///    - k even and zeros odd: parity can never match since m*k is always even
    ///    - k = n and 0 < zeros < n: flipping all bits cycles between states
    /// 2. Find minimum m where:
    ///    - m*k >= zeros (sufficient total flips)
    ///    - m*k ≡ zeros (mod 2) (parity matches)
    ///    - Upper bound satisfied based on flip distribution capacity
    ///
    /// # Complexity
    /// - Time: O(n) for counting zeros, O(n/k) for finding m
    /// - Space: O(1)
    pub fn min_operations(s: String, k: i32) -> i32 {
        let n = s.len() as i64;
        let k = k as i64;
        let zeros = s.bytes().filter(|&b| b == b'0').count() as i64;

        if zeros == 0 {
            return 0;
        }

        if k % 2 == 0 && zeros % 2 == 1 {
            return -1;
        }

        if k == n && zeros != n {
            return -1;
        }

        let mut m = (zeros + k - 1) / k;

        loop {
            let total = m * k;
            let parity_ok = total % 2 == zeros % 2;
            let upper_ok = if m % 2 == 1 {
                total <= n * (m - 1) + zeros
            } else {
                total <= n * m - zeros
            };

            if parity_ok && upper_ok {
                return m as i32;
            }

            m += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_zero_k_one() {
        assert_eq!(Solution::min_operations("110".to_string(), 1), 1);
    }

    #[test]
    fn test_alternating_k_three() {
        assert_eq!(Solution::min_operations("0101".to_string(), 3), 2);
    }

    #[test]
    fn test_impossible_parity() {
        assert_eq!(Solution::min_operations("101".to_string(), 2), -1);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::min_operations("111".to_string(), 2), 0);
    }

    #[test]
    fn test_all_zeros_k_equals_n() {
        assert_eq!(Solution::min_operations("000".to_string(), 3), 1);
    }

    #[test]
    fn test_impossible_k_equals_n() {
        assert_eq!(Solution::min_operations("001".to_string(), 3), -1);
    }

    #[test]
    fn test_upper_bound_constraint() {
        // n=5, k=4, zeros=2: m=1 violates upper bound, m=2 works
        assert_eq!(Solution::min_operations("11001".to_string(), 4), 2);
    }

    #[test]
    fn test_large_k_small_zeros() {
        assert_eq!(Solution::min_operations("10000".to_string(), 4), 1);
    }

    #[test]
    fn test_single_char_zero() {
        assert_eq!(Solution::min_operations("0".to_string(), 1), 1);
    }

    #[test]
    fn test_single_char_one() {
        assert_eq!(Solution::min_operations("1".to_string(), 1), 0);
    }
}
