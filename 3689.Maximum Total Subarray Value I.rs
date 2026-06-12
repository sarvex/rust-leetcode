impl Solution {
    /// Maximize total value of k chosen subarrays by repeating the globally optimal one.
    ///
    /// # Intuition
    /// The value of any subarray is max - min. The maximum achievable value for a single
    /// subarray is `global_max - global_min` (the subarray spanning both extremes). Since
    /// subarrays may overlap and repeat, choosing this optimal subarray all k times gives
    /// the maximum total value.
    ///
    /// # Approach
    /// 1. Find the global maximum and minimum of `nums` in a single pass.
    /// 2. Return `k * (global_max - global_min)` as `i64`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let (min, max) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(lo, hi), &x| (lo.min(x), hi.max(x)));
        k as i64 * (max - min) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_total_value(vec![7], 5), 0);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(Solution::max_total_value(vec![3, 3, 3], 4), 0);
    }

    #[test]
    fn test_large_k() {
        assert_eq!(
            Solution::max_total_value(vec![0, 1_000_000_000], 100_000),
            100_000 * 1_000_000_000_i64
        );
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::max_total_value(vec![0, 10], 1), 10);
    }
}
