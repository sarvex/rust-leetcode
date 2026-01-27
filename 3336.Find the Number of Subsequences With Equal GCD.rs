impl Solution {
    /// Uses 3-state DP tracking GCD pairs to count disjoint subsequence pairs.
    ///
    /// # Intuition
    /// Each element has three choices: join seq1, join seq2, or skip. We track
    /// the GCD of each subsequence as DP state. Since values are at most 200,
    /// GCD values are bounded, making a 2D GCD state feasible.
    ///
    /// # Approach
    /// 1. Define dp[g1][g2] = number of ways where seq1 has GCD g1 and seq2 has GCD g2.
    /// 2. Use g=0 to represent an empty subsequence (gcd(0,x) = x).
    /// 3. For each number, update DP by considering all three choices.
    /// 4. Sum dp[g][g] for all g >= 1 to count valid pairs.
    ///
    /// # Complexity
    /// - Time: O(n * M^2) where M = max(nums) = 200
    /// - Space: O(M^2)
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        const MAX_VAL: usize = 201;

        let mut dp = vec![vec![0u64; MAX_VAL]; MAX_VAL];
        dp[0][0] = 1;

        for &num in &nums {
            let v = num as usize;
            let mut new_dp = vec![vec![0u64; MAX_VAL]; MAX_VAL];

            for g1 in 0..MAX_VAL {
                for g2 in 0..MAX_VAL {
                    if dp[g1][g2] == 0 {
                        continue;
                    }
                    let ways = dp[g1][g2];

                    // Skip this element
                    new_dp[g1][g2] = (new_dp[g1][g2] + ways) % MOD;

                    // Add to seq1
                    let ng1 = Self::gcd(g1, v);
                    new_dp[ng1][g2] = (new_dp[ng1][g2] + ways) % MOD;

                    // Add to seq2
                    let ng2 = Self::gcd(g2, v);
                    new_dp[g1][ng2] = (new_dp[g1][ng2] + ways) % MOD;
                }
            }

            dp = new_dp;
        }

        let result = (1..MAX_VAL).fold(0u64, |acc, g| (acc + dp[g][g]) % MOD);

        result as i32
    }

    #[inline(always)]
    fn gcd(a: usize, b: usize) -> usize {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        let mut x = a;
        let mut y = b;
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::subsequence_pair_count(vec![5]), 0);
    }

    #[test]
    fn test_two_equal_elements() {
        assert_eq!(Solution::subsequence_pair_count(vec![3, 3]), 1);
    }

    #[test]
    fn test_coprime_elements() {
        assert_eq!(Solution::subsequence_pair_count(vec![2, 3]), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::subsequence_pair_count(vec![7, 7, 7]), 6);
    }
}
