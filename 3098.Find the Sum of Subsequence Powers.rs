const MOD: i64 = 1_000_000_007;

fn count_subsets_with_min_gap_at_least(values: &[i64], k: usize, gap: i64) -> i64 {
    let n = values.len();
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;

    let mut eligible: isize = -1;
    for i in 1..=n {
        while (eligible + 1) < (i as isize - 1)
            && values[i - 1] - values[(eligible + 1) as usize] >= gap
        {
            eligible += 1;
        }
        let allowed_prefix = (eligible + 1) as usize;
        dp[i][0] = 1;
        for j in 1..=k {
            let without = dp[i - 1][j];
            let with = dp[allowed_prefix][j - 1];
            dp[i][j] = (without + with) % MOD;
        }
    }

    dp[n][k]
}

impl Solution {
    /// Sum powers by counting subsets with bounded minimum gaps.
    ///
    /// # Intuition
    /// The minimum absolute difference of a sorted subset equals the smallest gap between
    /// adjacent chosen elements, so we can count subsets whose minimum gap is at least `g`.
    ///
    /// # Approach
    /// - Sort `nums`, because order does not affect the minimum difference of a chosen subset.
    /// - Collect all distinct pairwise differences `g`.
    /// - For each `g`, compute `F(g)`: the number of `k`-element subsets whose adjacent gaps
    ///   are all at least `g` using a prefix DP with a moving pointer for the last allowed index.
    /// - Subsets with minimum gap exactly `g` equal `F(g) - F(next_g)`, so accumulate
    ///   `g * (F(g) - F(next_g))` modulo `1e9 + 7`.
    ///
    /// # Complexity
    /// - Time: O(n^2 * k)
    /// - Space: O(n * k)
    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        let mut values: Vec<i64> = nums.into_iter().map(|value| value as i64).collect();
        values.sort_unstable();

        let n = values.len();
        let k = k as usize;
        let mut diffs = Vec::with_capacity(n.saturating_mul(n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                diffs.push(values[j] - values[i]);
            }
        }
        diffs.sort_unstable();
        diffs.dedup();
        if diffs.is_empty() {
            return 0;
        }

        let mut counts = Vec::with_capacity(diffs.len());
        for &gap in &diffs {
            counts.push(count_subsets_with_min_gap_at_least(&values, k, gap));
        }

        let mut total = 0i64;
        for idx in 0..diffs.len() {
            let current = counts[idx];
            let next = if idx + 1 < counts.len() { counts[idx + 1] } else { 0 };
            let mut diff = current - next;
            diff %= MOD;
            if diff < 0 {
                diff += MOD;
            }
            total = (total + (diffs[idx] % MOD) * diff) % MOD;
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::sum_of_powers(nums, 3), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 2];
        assert_eq!(Solution::sum_of_powers(nums, 2), 0);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![4, 3, -1];
        assert_eq!(Solution::sum_of_powers(nums, 2), 10);
    }

    #[test]
    fn test_all_equal() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(Solution::sum_of_powers(nums, 3), 0);
    }

    #[test]
    fn test_pairs_only() {
        let nums = vec![1, 4, 7];
        assert_eq!(Solution::sum_of_powers(nums, 2), 12);
    }
}
