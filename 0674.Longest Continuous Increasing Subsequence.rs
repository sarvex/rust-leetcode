impl Solution {
    /// Finds the longest continuous increasing subsequence length.
    ///
    /// # Intuition
    /// A single scan tracking the current streak length suffices since we
    /// need strictly increasing consecutive elements.
    ///
    /// # Approach
    /// 1. Start with streak = 1.
    /// 2. Extend when nums[i] > nums[i-1]; reset otherwise.
    /// 3. Track the maximum streak.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        nums.windows(2)
            .fold((1, 1), |(best, streak), w| {
                let s = if w[1] > w[0] { streak + 1 } else { 1 };
                (best.max(s), s)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::find_length_of_lcis(vec![1]), 1);
    }
}
