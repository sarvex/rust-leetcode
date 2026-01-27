impl Solution {
    /// Dynamic programming with GCD state tracking for coprime row selections.
    ///
    /// # Intuition
    /// Track all possible GCD values after selecting one element from each row.
    /// Since values are bounded by 150, we only need to track GCDs from 1 to 150.
    ///
    /// # Approach
    /// 1. Use DP where dp[g] = number of ways to achieve GCD = g
    /// 2. Initialize with first row elements
    /// 3. For each subsequent row, compute new GCDs by combining current states with row elements
    /// 4. Return dp[1] as the count of coprime combinations
    ///
    /// # Complexity
    /// - Time: O(m * n * MAX_VAL) where MAX_VAL = 150
    /// - Space: O(MAX_VAL)
    pub fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const MAX_VAL: usize = 151;

        fn gcd(a: usize, b: usize) -> usize {
            match b {
                0 => a,
                _ => gcd(b, a % b),
            }
        }

        let mut dp = vec![0i64; MAX_VAL];

        mat[0].iter().for_each(|&val| dp[val as usize] += 1);

        mat.iter().skip(1).for_each(|row| {
            let mut new_dp = vec![0i64; MAX_VAL];
            (1..MAX_VAL).filter(|&g| dp[g] > 0).for_each(|g| {
                row.iter().for_each(|&val| {
                    let new_g = gcd(g, val as usize);
                    new_dp[new_g] = (new_dp[new_g] + dp[g]) % MOD;
                });
            });
            dp = new_dp;
        });

        dp[1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_coprime_pairs() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::count_coprime(mat), 3);
    }

    #[test]
    fn test_no_coprime_combinations() {
        let mat = vec![vec![2, 2], vec![2, 2]];
        assert_eq!(Solution::count_coprime(mat), 0);
    }

    #[test]
    fn test_single_row_with_one() {
        let mat = vec![vec![1, 2, 3]];
        assert_eq!(Solution::count_coprime(mat), 1);
    }

    #[test]
    fn test_all_ones_maximize_combinations() {
        let mat = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(Solution::count_coprime(mat), 8);
    }

    #[test]
    fn test_coprime_prime_pairs() {
        let mat = vec![vec![2, 3], vec![5, 7]];
        assert_eq!(Solution::count_coprime(mat), 4);
    }
}
