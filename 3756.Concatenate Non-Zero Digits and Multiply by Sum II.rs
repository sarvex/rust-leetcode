impl Solution {
    /// Concatenates non-zero digits in range and multiplies by their sum.
    ///
    /// # Intuition
    /// Build prefix arrays over the compressed (non-zero) sequence, then map each original
    /// position to its nearest non-zero compressed index. This gives O(1) per query after
    /// O(n) preprocessing.
    ///
    /// # Approach
    /// 1. Scan `s` once to build:
    ///    - `prefix_sum[k]`: sum of the first k non-zero digits (mod MOD)
    ///    - `prefix_num[k]`: number formed by concatenating first k non-zero digits (mod MOD)
    ///    - `pows[k]`: 10^k mod MOD
    ///    - `nz_left[i]`: compressed index of the first non-zero digit at position >= i
    ///    - `nz_right[i]`: compressed index of the last non-zero digit at position <= i
    /// 2. For each query [l, r]:
    ///    - Map l → compressed left boundary via `nz_left[l]`
    ///    - Map r → compressed right boundary via `nz_right[r]`
    ///    - If the range is empty (all zeros), answer is 0
    ///    - Otherwise extract sub-number and sum using prefix arrays in O(1)
    ///
    /// # Complexity
    /// - Time: O(n + q)
    /// - Space: O(n)
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let bytes = s.as_bytes();
        let n = bytes.len();

        // Count non-zero digits for exact allocation
        let nz_count = bytes.iter().filter(|&&b| b != b'0').count();

        let mut prefix_sum = vec![0_i64; nz_count + 1];
        let mut prefix_num = vec![0_i64; nz_count + 1];
        let mut pows = vec![1_i64; nz_count + 1];

        // nz_left[i] = compressed index of the FIRST non-zero digit at position >= i
        // Use nz_count as sentinel meaning "no non-zero digit in [i, n)"
        let mut nz_left = vec![nz_count; n + 1];
        // nz_right[i] = compressed index of the LAST non-zero digit at position <= i
        // Use usize::MAX as sentinel meaning "no non-zero digit in [0, i]"
        let mut nz_right = vec![usize::MAX; n];

        // Forward pass: fill prefix arrays and nz_left
        let mut ci = 0_usize; // compressed index of next non-zero
        for i in 0..n {
            nz_left[i] = ci;
            if bytes[i] != b'0' {
                let d = i64::from(bytes[i] - b'0');
                prefix_num[ci + 1] = (prefix_num[ci] * 10 + d) % MOD;
                prefix_sum[ci + 1] = (prefix_sum[ci] + d) % MOD;
                pows[ci + 1] = pows[ci] * 10 % MOD;
                ci += 1;
            }
        }
        // nz_left[n] stays nz_count (sentinel)

        // Backward pass: fill nz_right
        let mut last_nz = usize::MAX; // sentinel: no non-zero seen yet from the right
        for i in (0..n).rev() {
            if bytes[i] != b'0' {
                last_nz = nz_left[i]; // compressed index of position i
            }
            nz_right[i] = last_nz;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for query in &queries {
            let l = query[0] as usize;
            let r = query[1] as usize;

            // Compressed range: [cl, cr] inclusive
            let cl = nz_left[l]; // first non-zero at or after l
            let cr = nz_right[r]; // last non-zero at or before r

            // Empty range: no non-zero digits in [l, r]
            if cl == nz_count || cr == usize::MAX || cl > cr {
                ans.push(0);
                continue;
            }

            // Sum of non-zero digits in compressed range [cl, cr]
            let sum = (prefix_sum[cr + 1] - prefix_sum[cl] + MOD) % MOD;

            // Number formed by concatenating those digits
            // = prefix_num[cr+1] - prefix_num[cl] * 10^(cr-cl+1)
            let len = cr + 1 - cl;
            let num = ((prefix_num[cr + 1] - prefix_num[cl] * pows[len]) % MOD + MOD) % MOD;

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

    #[test]
    fn test_query_spanning_zeros_at_both_ends() {
        // s = "0120", query [0, 3] -> non-zero digits are 1, 2 -> x = 12, sum = 3, result = 36
        let s = "0120".to_string();
        let queries = vec![vec![0, 3]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![36]);
    }

    #[test]
    fn test_query_on_single_nonzero() {
        // s = "030", query [1, 1] -> non-zero digit is 3 -> x = 3, sum = 3, result = 9
        let s = "030".to_string();
        let queries = vec![vec![1, 1]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![9]);
    }

    #[test]
    fn test_right_boundary_is_zero() {
        // s = "120", query [0, 2] -> non-zero digits 1, 2 -> x = 12, sum = 3, result = 36
        let s = "120".to_string();
        let queries = vec![vec![0, 2]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![36]);
    }

    #[test]
    fn test_left_boundary_is_zero() {
        // s = "021", query [0, 2] -> non-zero digits 2, 1 -> x = 21, sum = 3, result = 63
        let s = "021".to_string();
        let queries = vec![vec![0, 2]];
        assert_eq!(Solution::sum_and_multiply(s, queries), vec![63]);
    }
}
