impl Solution {
    /// Accumulate subset minimums while scanning increasing maxima.
    ///
    /// # Intuition
    /// After sorting, if `x` is the current maximum, any subset of previous elements can pair with
    /// it. The minimum of that subset is either `x` itself (when the subset is empty) or the
    /// minimum of the chosen previous subset. So we need the sum of minimums of all previous
    /// subsets.
    ///
    /// # Approach
    /// Sort `nums` ascending and maintain `sum_mins`, the sum of minimums across all non-empty
    /// subsets of processed values. For each value `x`:
    /// - The total minimum contribution for subsets where `x` is the maximum is `sum_mins + x`.
    /// - Add `x^2 * (sum_mins + x)` to the answer.
    /// - Update `sum_mins = 2 * sum_mins + x`, since each existing subset can be duplicated by
    ///   either including or excluding `x`, and the singleton `{x}` contributes `x`.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) extra (in-place sort)
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        nums.sort_unstable();

        let mut sum_mins: i64 = 0;
        let mut answer: i64 = 0;

        for value in nums {
            let x = value as i64;
            let x_sq = x * x % MOD;
            let contrib = x_sq * ((sum_mins + x) % MOD) % MOD;
            answer = (answer + contrib) % MOD;
            sum_mins = (sum_mins * 2 + x) % MOD;
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 1, 4];
        assert_eq!(Solution::sum_of_power(nums), 141);
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::sum_of_power(nums), 7);
    }

    #[test]
    fn single_value() {
        let nums = vec![5];
        assert_eq!(Solution::sum_of_power(nums), 125);
    }

    #[test]
    fn two_values() {
        let nums = vec![1, 2];
        assert_eq!(Solution::sum_of_power(nums), 13);
    }
}
