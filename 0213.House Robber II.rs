impl Solution {
    /// Maximizes robbery in a circular street by reducing to two linear subproblems.
    ///
    /// # Intuition
    /// Since the first and last houses are adjacent in a circle, we cannot rob both.
    /// Solve two linear House Robber problems: one excluding the last house and one
    /// excluding the first.
    ///
    /// # Approach
    /// 1. Handle the single-house edge case.
    /// 2. Compute max robbery for houses `[0..n-1]` and `[1..n]`.
    /// 3. Return the maximum of both.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let rob_range = |left: usize, right: usize| -> i32 {
            let (mut prev2, mut prev1) = (0, 0);
            for i in left..right {
                let current = prev1.max(prev2 + nums[i]);
                prev2 = prev1;
                prev1 = current;
            }
            prev1
        };
        rob_range(0, n - 1).max(rob_range(1, n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circular_robbery() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn optimal_skip_first() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn single_house() {
        assert_eq!(Solution::rob(vec![5]), 5);
    }

    #[test]
    fn two_houses() {
        assert_eq!(Solution::rob(vec![1, 2]), 2);
    }
}
