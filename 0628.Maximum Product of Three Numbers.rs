impl Solution {
    /// Sort-then-compare the two candidate triplets for the maximum product.
    ///
    /// # Intuition
    /// After sorting, the maximum product of three numbers is always one of two
    /// candidates:
    /// 1. The three largest values (top of the sorted array).
    /// 2. The two smallest values (most-negative) multiplied by the largest value —
    ///    two large negatives yield a large positive, which when multiplied by the
    ///    biggest element can beat three positives.
    ///
    /// # Approach
    /// Sort `nums` in ascending order. Let `n = nums.len()`. Compare:
    /// - `nums[n-1] * nums[n-2] * nums[n-3]`
    /// - `nums[0] * nums[1] * nums[n-1]`
    ///
    /// Return the larger of the two.
    ///
    /// # Complexity
    /// - Time: O(n log n) — dominated by sorting
    /// - Space: O(1) — sort is in-place; no extra allocation
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (nums[n - 1] * nums[n - 2] * nums[n - 3]).max(nums[0] * nums[1] * nums[n - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_positives() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
    }

    #[test]
    fn test_four_positives() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    }

    #[test]
    fn test_all_negatives() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
    }

    #[test]
    fn test_two_negatives_win() {
        // Two largest negatives × largest positive: (-10)*(-10)*5 = 500
        // Three largest: 1*2*5 = 10
        assert_eq!(Solution::maximum_product(vec![-10, -10, 1, 2, 5]), 500);
    }

    #[test]
    fn test_mixed_positive_negative() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3, 4]), 24);
    }

    #[test]
    fn test_contains_zeros() {
        assert_eq!(Solution::maximum_product(vec![0, 0, 0, 1]), 0);
    }

    #[test]
    fn test_boundary_values() {
        // Max i32-safe triple: 1000 * 1000 * 1000 = 1_000_000_000
        assert_eq!(
            Solution::maximum_product(vec![-1000, -1000, 1000, 1000]),
            1_000_000_000
        );
    }

    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::maximum_product(vec![-1000, 0, 1000]), 0);
    }
}
