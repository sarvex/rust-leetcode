impl Solution {
    /// Computes minimum elements (each ≤ limit) needed to reach the goal sum.
    ///
    /// # Intuition
    /// The deficit between the current sum and goal must be covered by values
    /// each at most `limit`. The number of elements is `ceil(|deficit| / limit)`.
    ///
    /// # Approach
    /// 1. Sum all elements using i64 to avoid overflow.
    /// 2. Compute the absolute difference from goal.
    /// 3. Return `ceil(diff / limit)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let diff = (goal as i64 - sum).abs();
        let limit = limit as i64;
        ((diff + limit - 1) / limit) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::min_elements(vec![1, -1, 1], 3, -4), 2);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::min_elements(vec![1, -10, 9, 1], 100, 0), 1);
    }
}
