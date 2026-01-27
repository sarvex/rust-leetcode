impl Solution {
    /// Finds minimum moves to make all elements equal using the median.
    ///
    /// # Intuition
    /// The optimal target minimizing total absolute deviations is the median.
    /// Each move changes one element by 1, so the answer is the sum of
    /// absolute differences from the median.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Select the median element.
    /// 3. Sum absolute differences from the median.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let median = nums[nums.len() / 2];
        nums.iter().map(|&x| (x - median).abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_already_equal() {
        assert_eq!(Solution::min_moves2(vec![5, 5, 5]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_moves2(vec![1, 10]), 9);
    }
}
