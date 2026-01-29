impl Solution {
    /// Counts possible ways to assign performers to stages and award scores.
    ///
    /// # Intuition
    /// For k non-empty stages, we need: choose k stages from x, distribute n
    /// performers into exactly k non-empty groups, then assign scores to each band.
    ///
    /// # Approach
    /// Sum over k from 1 to min(n,x) of: P(x,k) × S(n,k) × y^k
    /// where P(x,k) is permutation (choosing and ordering k stages from x)
    /// and S(n,k) is Stirling number of second kind (partitioning n into k groups).
    ///
    /// # Complexity
    /// - Time: O(n × min(n,x))
    /// - Space: O(n × min(n,x))
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let x = x as usize;
        let y = y as i64;

        let max_k = n.min(x);

        // Precompute Stirling numbers of the second kind S(n, k)
        // S(n, k) = k * S(n-1, k) + S(n-1, k-1)
        let mut stirling = vec![vec![0i64; max_k + 1]; n + 1];
        stirling[0][0] = 1;
        for i in 1..=n {
            for j in 1..=i.min(max_k) {
                stirling[i][j] = (j as i64 * stirling[i - 1][j] + stirling[i - 1][j - 1]) % MOD;
            }
        }

        let mut result = 0i64;
        let mut perm = 1i64; // P(x, k) = x × (x-1) × ... × (x-k+1)
        let mut pow_y = 1i64;

        for k in 1..=max_k {
            perm = perm * (x - k + 1) as i64 % MOD;
            pow_y = pow_y * y % MOD;
            let contribution = perm * stirling[n][k] % MOD * pow_y % MOD;
            result = (result + contribution) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::number_of_ways(1, 2, 3), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::number_of_ways(5, 2, 1), 32);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::number_of_ways(3, 3, 4), 684);
    }

    #[test]
    fn test_single_stage() {
        // n=3, x=1, y=2: all performers on 1 stage, 2 score choices
        assert_eq!(Solution::number_of_ways(3, 1, 2), 2);
    }

    #[test]
    fn test_single_score() {
        // n=2, x=3, y=1: each config has only 1 score choice per band
        // k=1: P(3,1)=3, S(2,1)=1, y^1=1 → 3
        // k=2: P(3,2)=6, S(2,2)=1, y^2=1 → 6
        assert_eq!(Solution::number_of_ways(2, 3, 1), 9);
    }

    #[test]
    fn test_large_values() {
        // Verify no overflow with larger inputs
        let result = Solution::number_of_ways(1000, 1000, 1000);
        assert!(result >= 0);
    }
}
