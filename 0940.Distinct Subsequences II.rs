impl Solution {
    /// Counts distinct non-empty subsequences modulo 10^9 + 7.
    ///
    /// # Intuition
    /// Track the number of new subsequences each character adds. Using an
    /// array for each letter avoids counting duplicates.
    ///
    /// # Approach
    /// For each character, the new count for that letter becomes the total
    /// of all existing subsequences plus 1 (the character alone). Sum all
    /// 26 entries for the final answer.
    ///
    /// # Complexity
    /// - Time: O(26 * n) = O(n)
    /// - Space: O(1) — fixed 26-element array
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = [0i64; 26];
        for b in s.bytes() {
            let idx = (b - b'a') as usize;
            dp[idx] = (dp.iter().sum::<i64>() + 1) % MOD;
        }
        (dp.iter().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc() {
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
    }

    #[test]
    fn test_aba() {
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
    }

    #[test]
    fn test_aaa() {
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }
}
