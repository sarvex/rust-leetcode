impl Solution {
    /// Computes the sum of floor(nums[i] / nums[j]) for all pairs.
    ///
    /// # Intuition
    /// For each distinct value y, the contribution at multiplier d is
    /// d * count(y) * count_in_range([d*y, d*y + y - 1]). A prefix sum
    /// over value frequencies enables O(1) range queries.
    ///
    /// # Approach
    /// 1. Count frequency of each value and build a prefix sum array.
    /// 2. For each value y with nonzero count, iterate over multipliers d.
    /// 3. For each d, add d * freq(y) * (prefix_sum range count).
    /// 4. Return the total modulo 10^9 + 7.
    ///
    /// # Complexity
    /// - Time: O(max_val * H(max_val)) ≈ O(max_val * log(max_val))
    /// - Space: O(max_val)
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let max_val = *nums.iter().max().unwrap() as usize;

        let mut count = vec![0i64; max_val + 1];
        for &x in &nums {
            count[x as usize] += 1;
        }

        let mut prefix = vec![0i64; max_val + 1];
        for i in 1..=max_val {
            prefix[i] = prefix[i - 1] + count[i];
        }

        let mut result = 0i64;
        for y in 1..=max_val {
            if count[y] > 0 {
                let mut d = 1;
                while d * y <= max_val {
                    let hi = (d * y + y - 1).min(max_val);
                    result =
                        (result + count[y] * d as i64 * (prefix[hi] - prefix[d * y - 1])) % MOD;
                    d += 1;
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::sum_of_floored_pairs(vec![2, 5, 9]), 10);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(
            Solution::sum_of_floored_pairs(vec![7, 7, 7, 7, 7, 7, 7]),
            49
        );
    }
}
