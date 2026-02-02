impl Solution {
    /// Minimizes changes by precomputing semi-palindrome costs and partitioning with DP.
    ///
    /// # Intuition
    /// A substring is semi-palindromic if some divisor `d` splits it into `d` residue groups and
    /// each group is a palindrome. For a fixed `d`, the minimum changes equal the sum of mismatched
    /// mirrored pairs within each group, so we can try every divisor and keep the best cost.
    ///
    /// # Approach
    /// - Precompute all proper divisors for lengths up to `n`.
    /// - For every substring `s[l..r)`, compute the minimum change cost across its divisors.
    /// - Use DP where `dp[i][p]` is the minimum changes to split the prefix `s[0..i)` into `p`
    ///   substrings; transition by trying all previous split points.
    ///
    /// # Complexity
    /// - Time: O(n^3), from cost precomputation and partition DP
    /// - Space: O(n^2)
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        const INF: i32 = 1_000_000;

        let bytes = s.as_bytes();
        let n = bytes.len();
        let parts = k as usize;

        let mut divisors = vec![Vec::<usize>::new(); n + 1];
        for d in 1..=n {
            let mut multiple = d * 2;
            while multiple <= n {
                divisors[multiple].push(d);
                multiple += d;
            }
        }

        let mut change = vec![vec![INF; n + 1]; n];
        for len in 2..=n {
            for start in 0..=n - len {
                let end = start + len;
                let mut best = INF;
                for &divisor in &divisors[len] {
                    let cost = Self::divisor_cost(bytes, start, len, divisor);
                    if cost < best {
                        best = cost;
                    }
                }
                change[start][end] = best;
            }
        }

        let mut dp = vec![vec![INF; parts + 1]; n + 1];
        dp[0][0] = 0;
        for part in 1..=parts {
            for end in 1..=n {
                let mut best = INF;
                for start in 0..end {
                    let cost = change[start][end];
                    if cost == INF {
                        continue;
                    }
                    let prev = dp[start][part - 1];
                    if prev == INF {
                        continue;
                    }
                    let candidate = prev + cost;
                    if candidate < best {
                        best = candidate;
                    }
                }
                dp[end][part] = best;
            }
        }

        dp[n][parts]
    }

    fn divisor_cost(bytes: &[u8], start: usize, len: usize, divisor: usize) -> i32 {
        let group_len = len / divisor;
        let mut changes = 0_i32;
        for offset in 0..divisor {
            let mut left = 0;
            let mut right = group_len - 1;
            while left < right {
                let left_idx = start + offset + left * divisor;
                let right_idx = start + offset + right * divisor;
                if bytes[left_idx] != bytes[right_idx] {
                    changes += 1;
                }
                left += 1;
                right -= 1;
            }
        }
        changes
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_changes("abcac".into(), 2), 1);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_changes("abcdef".into(), 2), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_changes("aabbaa".into(), 3), 0);
    }

    #[test]
    fn test_repeating_pattern() {
        assert_eq!(Solution::minimum_changes("abcabc".into(), 1), 0);
    }

    #[test]
    fn test_all_same_values() {
        assert_eq!(Solution::minimum_changes("aaaaaa".into(), 3), 0);
    }

    #[test]
    fn test_two_chars() {
        assert_eq!(Solution::minimum_changes("az".into(), 1), 1);
    }
}
