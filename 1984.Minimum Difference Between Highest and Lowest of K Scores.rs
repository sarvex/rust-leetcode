impl Solution {
    /// Sliding window approach: sort array and find minimum difference in consecutive k-sized windows
    ///
    /// # Intuition
    /// After sorting, the minimum difference between highest and lowest scores in any k-sized group
    /// will occur in consecutive elements. We can use a sliding window to check all possible k-sized
    /// windows efficiently.
    ///
    /// # Approach
    /// 1. Sort the array to bring similar values together
    /// 2. Use a sliding window of size k to iterate through consecutive elements
    /// 3. Calculate the difference between the maximum (last) and minimum (first) in each window
    /// 4. Track the minimum difference found across all windows
    ///
    /// # Complexity
    /// Time complexity: O(n log n) where n is the length of nums (dominated by sorting)
    /// Space complexity: O(1) excluding the input array (sorting may use O(log n) stack space)
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let window_size = k as usize;

        (0..=nums.len().saturating_sub(window_size))
            .map(|start_idx| nums[start_idx + window_size - 1] - nums[start_idx])
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_difference(vec![5], 1), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::minimum_difference(vec![3, 3, 3, 3], 2), 0);
    }

    #[test]
    fn test_large_difference() {
        assert_eq!(
            Solution::minimum_difference(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
            2
        );
    }
}
