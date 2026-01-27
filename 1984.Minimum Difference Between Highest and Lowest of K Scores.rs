impl Solution {
    /// Finds the minimum difference between highest and lowest of k scores.
    ///
    /// # Intuition
    /// After sorting, the optimal k-sized group consists of consecutive
    /// elements. A sliding window finds the minimum range.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Slide a window of size k, computing last - first for each.
    /// 3. Return the minimum difference.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let w = k as usize;

        (0..=nums.len().saturating_sub(w))
            .map(|i| nums[i + w - 1] - nums[i])
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn test_basic() {
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::minimum_difference(vec![3, 3, 3, 3], 2), 0);
    }

    #[test]
    fn test_larger_window() {
        assert_eq!(
            Solution::minimum_difference(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3),
            2
        );
    }
}
