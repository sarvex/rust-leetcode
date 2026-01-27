impl Solution {
    /// Finds the smallest possible range after adding [-k, k] to each element.
    ///
    /// # Intuition
    /// The minimum range is `max - min - 2*k`, clamped to zero.
    ///
    /// # Approach
    /// Compute max and min of the array. The answer is `max(0, max - min - 2*k)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        (max - min - 2 * k).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    }

    #[test]
    fn test_range_reduced() {
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    }

    #[test]
    fn test_range_zero() {
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
