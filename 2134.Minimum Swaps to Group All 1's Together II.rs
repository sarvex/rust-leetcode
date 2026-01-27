impl Solution {
    /// Finds minimum swaps to group all 1s together in a circular array.
    ///
    /// # Intuition
    /// The number of 1s determines the required window size. By sliding a
    /// window of that size across the circular array, the window with the
    /// most 1s requires the fewest swaps (swap count = window size - ones
    /// in window).
    ///
    /// # Approach
    /// 1. Count total 1s (`k`) — this is the window size.
    /// 2. Count 1s in the initial window `[0..k)`.
    /// 3. Slide the window circularly, updating the count and tracking
    ///    the maximum number of 1s found in any window.
    /// 4. Answer is `k - max_ones_in_window`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let k: usize = nums.iter().map(|&x| x as usize).sum();
        if k == 0 {
            return 0;
        }

        let mut ones_in_window: i32 = nums[..k].iter().sum();
        let mut max_ones = ones_in_window;

        for i in k..n + k {
            ones_in_window += nums[i % n] - nums[(i - k) % n];
            max_ones = max_ones.max(ones_in_window);
        }

        k as i32 - max_ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    }

    #[test]
    fn test_already_grouped() {
        assert_eq!(Solution::min_swaps(vec![1, 1, 1, 0, 0]), 0);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(Solution::min_swaps(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(Solution::min_swaps(vec![1, 1, 1]), 0);
    }

    #[test]
    fn test_circular_wrap() {
        assert_eq!(Solution::min_swaps(vec![1, 0, 0, 0, 1]), 0);
    }
}
