impl Solution {
    /// Counts substrings where the first and last characters are the same per query.
    ///
    /// # Intuition
    /// For each character, the number of same-end substrings is the count of single
    /// characters plus the number of pairs (C(freq, 2)). Prefix sums per character
    /// allow efficient range frequency queries.
    ///
    /// # Approach
    /// 1. Build a 26 × (n+1) prefix count array over `as_bytes()`.
    /// 2. For each query `[l, r]`, compute the range frequency for each character.
    /// 3. Add `(r - l + 1)` for single-character substrings, plus `x*(x-1)/2` for
    ///    each character with frequency `x` in the range.
    ///
    /// # Complexity
    /// - Time: O(26 * n + 26 * q) where q is the number of queries
    /// - Space: O(26 * n) for the prefix array
    pub fn same_end_substring_count(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut cnt = vec![vec![0i32; n + 1]; 26];

        for j in 1..=n {
            for i in 0..26 {
                cnt[i][j] = cnt[i][j - 1];
            }
            cnt[(bytes[j - 1] - b'a') as usize][j] += 1;
        }

        queries
            .iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                let base = (r - l + 1) as i32;
                let pairs: i32 = (0..26)
                    .map(|i| {
                        let x = cnt[i][r + 1] - cnt[i][l];
                        x * (x - 1) / 2
                    })
                    .sum();
                base + pairs
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_queries() {
        let result = Solution::same_end_substring_count(
            "abcaab".to_string(),
            vec![vec![0, 0], vec![1, 4], vec![0, 5]],
        );
        assert_eq!(result, vec![1, 5, 10]);
    }

    #[test]
    fn test_single_char_string() {
        let result = Solution::same_end_substring_count("aaa".to_string(), vec![vec![0, 2]]);
        assert_eq!(result, vec![6]);
    }
}
