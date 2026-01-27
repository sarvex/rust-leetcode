impl Solution {
    /// Finds the maximum sum of a strictly ascending contiguous subarray.
    ///
    /// # Intuition
    /// Track a running sum that resets whenever the ascending order breaks.
    /// The answer is the maximum running sum observed.
    ///
    /// # Approach
    /// 1. Initialize running sum with the first element.
    /// 2. For each subsequent element, reset sum if not strictly ascending.
    /// 3. Accumulate and track the maximum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut current = nums[0];

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                result = result.max(current);
                current = 0;
            }
            current += nums[i];
        }

        result.max(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_subarray() {
        assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_ascending_sum(vec![100]), 100);
    }

    #[test]
    fn test_descending() {
        assert_eq!(Solution::max_ascending_sum(vec![5, 4, 3, 2, 1]), 5);
    }
}
