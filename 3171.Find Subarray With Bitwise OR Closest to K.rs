use std::collections::HashSet;

impl Solution {
    /// For each right endpoint, maintain distinct OR values of subarrays ending there; at most
    /// O(bits) distinct values. Minimize |k - or_val| over all such ORs.
    ///
    /// # Intuition
    /// Extending a subarray to the right only adds bits to the OR (never removes). So for a fixed
    /// right index, the set of OR values over all left endpoints has size at most ~30 (one per bit).
    ///
    /// # Approach
    /// - Maintain a set `or_values` of distinct OR values for subarrays ending at the current index.
    /// - For each new element `x`: add `x` and `prev_or | x` for each `prev_or` in the set; then
    ///   replace the set with these new values (deduped).
    /// - After each step, update the answer as the minimum of `|k - or_val|` over the set.
    ///
    /// # Complexity
    /// - Time: O(n * B) where B ≤ 30 is the number of distinct ORs per step.
    /// - Space: O(B)
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = (k - nums[0]).abs();
        let mut or_values: HashSet<i32> = HashSet::from([nums[0]]);

        for &num in nums.iter().skip(1) {
            let next_set: HashSet<i32> = or_values
                .iter()
                .map(|&prev_or| prev_or | num)
                .chain(std::iter::once(num))
                .collect();
            or_values = next_set;
            result = or_values
                .iter()
                .map(|&or_val| (k - or_val).abs())
                .min()
                .unwrap_or(result)
                .min(result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::minimum_difference(vec![1, 2, 4, 5], 3), 0);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::minimum_difference(vec![1, 3, 1, 3], 2), 1);
    }

    #[test]
    fn test_example_three() {
        assert_eq!(Solution::minimum_difference(vec![1], 10), 9);
    }

    #[test]
    fn test_single_match() {
        assert_eq!(Solution::minimum_difference(vec![7], 7), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::minimum_difference(vec![2, 1], 3), 0);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(Solution::minimum_difference(vec![1, 2, 3], 1_000_000_000), 999_999_997);
    }
}
