impl Solution {
    /// Finds the maximum absolute subarray sum using dual Kadane's algorithm.
    ///
    /// # Intuition
    /// The maximum absolute sum is either the maximum subarray sum or the absolute
    /// value of the minimum subarray sum. Track both simultaneously.
    ///
    /// # Approach
    /// 1. Maintain running max subarray `f` and running min subarray `g`.
    /// 2. At each element, update both using Kadane's recurrence.
    /// 3. Return the maximum of `f` and `-g` seen so far.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0, 0), |(f, g, ans), &x| {
                let new_f = f.max(0) + x;
                let new_g = g.min(0) + x;
                (new_f, new_g, ans.max(new_f).max(-new_g))
            })
            .2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_absolute_sum(vec![-7]), 7);
    }
}
