impl Solution {
    /// Computes sum of all good subsequences via hash-map DP.
    ///
    /// # Intuition
    /// A good subsequence requires consecutive elements to differ by exactly 1.
    /// For each element we can start a new subsequence or extend ones ending at
    /// adjacent values. Track both count and total sum per ending value.
    ///
    /// # Approach
    /// 1. Maintain `count[v]` (number of subsequences ending with v) and
    ///    `total_sum[v]` (element sum across those subsequences).
    /// 2. For each x, compute new contributions from x-1 and x+1 neighbors.
    /// 3. Accumulate each element's sum contribution into the result.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of nums
    /// - Space: O(m) where m is the number of distinct values
    pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 1_000_000_007;

        let mut count: HashMap<i32, i64> = HashMap::new();
        let mut total_sum: HashMap<i32, i64> = HashMap::new();
        let mut result = 0i64;

        for x in nums {
            let x_val = x as i64;

            let count_prev = *count.get(&(x - 1)).unwrap_or(&0);
            let count_next = *count.get(&(x + 1)).unwrap_or(&0);
            let sum_prev = *total_sum.get(&(x - 1)).unwrap_or(&0);
            let sum_next = *total_sum.get(&(x + 1)).unwrap_or(&0);

            let new_count = (count_prev + count_next + 1) % MOD;
            let new_sum = (sum_prev + sum_next + x_val * new_count % MOD) % MOD;

            *count.entry(x).or_insert(0) = (count.get(&x).unwrap_or(&0) + new_count) % MOD;
            *total_sum.entry(x).or_insert(0) = (total_sum.get(&x).unwrap_or(&0) + new_sum) % MOD;

            result = (result + new_sum) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_elements_produce_combined_sums() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![1, 2, 1]), 14);
    }

    #[test]
    fn strictly_increasing_sequence() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![3, 4, 5]), 40);
    }

    #[test]
    fn single_element_returns_itself() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![5]), 5);
    }

    #[test]
    fn non_adjacent_values_yield_individual_sums() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![1, 3, 5]), 9);
    }

    #[test]
    fn all_identical_elements_sum_individually() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![2, 2, 2]), 6);
    }

    #[test]
    fn sequence_starting_with_zero() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![0, 1]), 2);
    }
}
