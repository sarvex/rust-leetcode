impl Solution {
    /// Finds maximum robbery amount using bottom-up DP with space optimization.
    ///
    /// # Intuition
    /// At each house, choose the maximum of robbing it (plus two houses back)
    /// or skipping it (previous house total). Only two previous values are needed.
    ///
    /// # Approach
    /// 1. Track two variables: `prev2` (two steps back) and `prev1` (one step back).
    /// 2. For each house, compute `max(prev1, nums[i] + prev2)`.
    /// 3. Shift the window forward.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(prev2, prev1), &num| {
                (prev1, prev1.max(prev2 + num))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternating_optimal() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn skip_middle() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
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
