impl Solution {
    /// Concatenates non-zero digits in range and multiplies by their sum.
    ///
    /// # Intuition
    /// Instead of compressing and using binary search, we use direct index mapping.
    /// Each position maps to its index in the compressed non-zero array, enabling O(1) lookup.
    ///
    /// # Approach
    /// 1. Build `indexes` array: for each position, store its compressed index
    /// 2. Build prefix sums and prefix "number" values on compressed sequence
    /// 3. Precompute powers of 10 for modular arithmetic
    /// 4. For each query, directly get compressed range and compute result
    ///
    /// # Complexity
    /// - Time: O(n + q) where n = s.len(), q = queries.len()
    /// - Space: O(n)
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        const BASE: i64 = 10;

        let len = s.len();
        let mut indexes = vec![0_usize; len];
        let mut prefix_sum = vec![0_i64; len + 1];
        let mut prefix_num = vec![0_i64; len + 1];
        let mut pows = vec![1_i64; len + 1];
        let mut compressed_idx = 0_usize;
        let bytes = s.as_bytes();

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'0' {
                indexes[i] = compressed_idx;
            } else {
                let digit = i64::from(byte - b'0');
                prefix_num[compressed_idx + 1] =
                    (prefix_num[compressed_idx] * BASE + digit) % MOD;
                prefix_sum[compressed_idx + 1] =
                    (prefix_sum[compressed_idx] + digit) % MOD;
                pows[compressed_idx + 1] = pows[compressed_idx] * BASE % MOD;
                indexes[i] = compressed_idx;
                compressed_idx += 1;
            }
        }

        let mut ans = Vec::with_capacity(queries.len());
        for query in &queries {
            let left = indexes[query[0] as usize];
            // If query[1] points to '0', we need the previous compressed index
            let right = indexes[query[1] as usize]
                .wrapping_sub(usize::from(bytes[query[1] as usize] == b'0'));

            // Check if range is empty (all zeros in query range)
            if right == usize::MAX || left > right {
                ans.push(0);
                continue;
            }

            // Sum of digits in compressed range [left, right]
            let sum = (prefix_sum[right + 1] - prefix_sum[left] + MOD) % MOD;

            // Number formed by digits in compressed range [left, right]
            // num = prefix_num[right+1] - prefix_num[left] * 10^(right-left+1)
            let num = ((prefix_num[right + 1] - prefix_num[left] * pows[right + 1 - left]) % MOD
                + MOD)
                % MOD;

            ans.push((sum * num % MOD) as i32);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "10203004".to_string();
        let queries = vec![vec![0, 7], vec![1, 3], vec![4, 6]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![12340, 4, 9]);
    }

    #[test]
    fn test_example_2() {
        let s = "1000".to_string();
        let queries = vec![vec![0, 3], vec![1, 1]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![1, 0]);
    }

    #[test]
    fn test_example_3() {
        let s = "9876543210".to_string();
        let queries = vec![vec![0, 9]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![444444137]);
    }

    #[test]
    fn test_single_zero() {
        let s = "0".to_string();
        let queries = vec![vec![0, 0]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![0]);
    }

    #[test]
    fn test_all_zeros() {
        let s = "00000".to_string();
        let queries = vec![vec![0, 4], vec![1, 3]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![0, 0]);
    }

    #[test]
    fn test_all_nonzero() {
        let s = "123".to_string();
        let queries = vec![vec![0, 2]];
        // x = 123, sum = 6, result = 738
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![738]);
    }

    #[test]
    fn test_single_digit_nonzero() {
        let s = "5".to_string();
        let queries = vec![vec![0, 0]];
        // x = 5, sum = 5, result = 25
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![25]);
    }
}
