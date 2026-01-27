impl Solution {
    /// Minimize the score by changing at most two elements.
    ///
    /// # Intuition
    /// The score is the sum of max-difference and min-difference. After sorting,
    /// changing two elements optimally means removing two extreme values from
    /// consideration: both from the top, both from the bottom, or one from each.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Consider three cases: remove two largest, remove two smallest,
    ///    or remove one from each end
    /// 3. Return the minimum of these three scores
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary (in-place sort)
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (nums[n - 1] - nums[2])
            .min(nums[n - 2] - nums[1])
            .min(nums[n - 3] - nums[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::minimize_sum(vec![1, 4, 3]), 0);
    }

    #[test]
    fn test_larger() {
        assert_eq!(Solution::minimize_sum(vec![1, 4, 7, 8, 5]), 3);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::minimize_sum(vec![5, 5, 5, 5]), 0);
    }
}
