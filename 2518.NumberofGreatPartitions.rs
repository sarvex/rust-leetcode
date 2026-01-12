impl Solution {
    /// Counts the number of great partitions of an array.
    ///
    /// # Intuition
    /// Use complementary counting: valid = total - invalid.
    /// A partition is invalid if either group has sum < k.
    ///
    /// # Approach
    /// 1. Early exit if total sum < 2k (impossible for both groups to reach k)
    /// 2. Count subsets with sum < k using O(k) space in-place DP
    /// 3. By symmetry, invalid = 2 × (subsets with sum < k)
    /// 4. Valid = 2^n - invalid
    ///
    /// # Complexity
    /// - Time: O(n × k)
    /// - Space: O(k)
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let k = k as i64;
        let total_sum: i64 = nums.iter().fold(0i64, |acc, &x| acc + x as i64);

        if total_sum < k << 1 {
            return 0;
        }

        let k = k as usize;
        let mut dp = vec![0i64; k];
        dp[0] = 1;

        for &num in &nums {
            let num = num as usize;
            if num < k {
                let mut j = k - 1;
                while j >= num {
                    dp[j] += dp[j - num];
                    if dp[j] >= MOD {
                        dp[j] -= MOD;
                    }
                    j -= 1;
                }
            }
        }

        let mut invalid = 0i64;
        let mut i = 0;
        while i < k {
            invalid += dp[i];
            i += 1;
        }
        invalid %= MOD;

        let mut total = 1i64;
        let mut i = nums.len();
        while i > 0 {
            total <<= 1;
            if total >= MOD {
                total -= MOD;
            }
            i -= 1;
        }

        let result = total - ((invalid << 1) % MOD);
        ((result % MOD + MOD) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_partitions(vec![1, 2, 3, 4], 4), 6);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_partitions(vec![3, 3, 3], 4), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::count_partitions(vec![6, 6], 2), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_partitions(vec![10], 5), 0);
    }

    #[test]
    fn test_sum_too_small() {
        assert_eq!(Solution::count_partitions(vec![1, 1, 1], 5), 0);
    }

    #[test]
    fn test_large_elements() {
        assert_eq!(Solution::count_partitions(vec![100, 100], 50), 2);
    }

    #[test]
    fn test_k_equals_one() {
        assert_eq!(Solution::count_partitions(vec![1, 1], 1), 2);
    }
}
