use std::collections::HashMap;

impl Solution {
    /// Finds minimum cost to partition a binary string via recursive midpoint splits.
    ///
    /// # Intuition
    /// We start with the entire string as one segment. The only operation is splitting
    /// an even-length segment into two equal halves. This forms a binary recursion tree
    /// where each node decides: keep whole or split.
    ///
    /// # Approach
    /// - Build prefix sums for O(1) range ones-count queries
    /// - Memoized recursion from `[0, n)`: at each segment, compare the unsplit cost
    ///   against the sum of recursively solving both halves (even-length only)
    /// - Only O(n) unique segments exist (binary tree of halvings), not O(n²)
    ///
    /// # Complexity
    /// - Time: O(n) — each segment in the halving tree is visited once
    /// - Space: O(n) for prefix sums and memoization
    pub fn min_cost(s: String, enc_cost: i32, flat_cost: i32) -> i64 {
        let n = s.len();
        let bytes = s.as_bytes();
        let enc = enc_cost as i64;
        let flat = flat_cost as i64;

        let mut prefix = vec![0i32; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + if bytes[i] == b'1' { 1 } else { 0 };
        }

        let mut memo = HashMap::new();

        fn solve(
            l: usize,
            r: usize,
            prefix: &[i32],
            enc: i64,
            flat: i64,
            memo: &mut HashMap<(usize, usize), i64>,
        ) -> i64 {
            if let Some(&v) = memo.get(&(l, r)) {
                return v;
            }

            let len = (r - l) as i64;
            let ones = (prefix[r] - prefix[l]) as i64;
            let base = if ones == 0 { flat } else { len * ones * enc };

            let result = if (r - l).is_multiple_of(2) {
                let m = l + (r - l) / 2;
                let split =
                    solve(l, m, prefix, enc, flat, memo) + solve(m, r, prefix, enc, flat, memo);
                base.min(split)
            } else {
                base
            };

            memo.insert((l, r), result);
            result
        }

        solve(0, n, &prefix, enc, flat, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_cost("1010".to_string(), 2, 1), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_cost("1010".to_string(), 3, 10), 12);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::min_cost("00".to_string(), 1, 2), 2);
    }

    #[test]
    fn test_failing_case() {
        assert_eq!(Solution::min_cost("010".to_string(), 22, 8), 66);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::min_cost("0000".to_string(), 5, 3), 3);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::min_cost("1111".to_string(), 2, 10), 8);
    }

    #[test]
    fn test_single_char_one() {
        assert_eq!(Solution::min_cost("1".to_string(), 1, 100), 1);
    }

    #[test]
    fn test_single_char_zero() {
        assert_eq!(Solution::min_cost("0".to_string(), 1, 5), 5);
    }

    #[test]
    fn test_split_cheaper() {
        assert_eq!(Solution::min_cost("11".to_string(), 1, 10), 2);
    }

    #[test]
    fn test_enc_cost_high_flat_cost_low() {
        assert_eq!(Solution::min_cost("1010".to_string(), 100, 1), 202);
    }

    #[test]
    fn test_odd_length_no_split() {
        // "101" length 3 (odd) → cannot split → 3 * 2 * 2 = 12
        assert_eq!(Solution::min_cost("101".to_string(), 2, 1), 12);
    }

    #[test]
    fn test_power_of_two_length() {
        // "11111111" → split all the way to singles: 8 * (1*1*1) = 8
        let s = "1".repeat(8);
        assert_eq!(Solution::min_cost(s, 1, 100), 8);
    }
}
