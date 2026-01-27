impl Solution {
    /// Minimizes the maximum pair sum by pairing smallest with largest elements.
    ///
    /// # Intuition
    /// Sorting and pairing extremes ensures no single pair dominates.
    /// The greedy strategy balances large values with small ones.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Pair the i-th smallest with the i-th largest.
    /// 3. Return the maximum pair sum.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort_unstable();

        let n = sorted.len();
        (0..n / 2)
            .map(|i| sorted[i] + sorted[n - 1 - i])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_pair_sum(vec![1, 2]), 3);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::min_pair_sum(vec![5, 5, 5, 5]), 10);
    }
}
