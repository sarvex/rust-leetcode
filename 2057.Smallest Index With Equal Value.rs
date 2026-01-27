impl Solution {
    /// Finds the smallest index where i % 10 == nums[i].
    ///
    /// # Intuition
    /// Linear scan returns the first index satisfying the condition.
    ///
    /// # Approach
    /// 1. Enumerate elements and find the first where index mod 10 equals value.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .find(|&(i, &x)| i % 10 == x as usize)
            .map_or(-1, |(i, _)| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
    }

    #[test]
    fn test_not_found() {
        assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
    }
}
