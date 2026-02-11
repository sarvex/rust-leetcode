impl Solution {
    /// Count elements with at least k strictly greater values using quickselect.
    ///
    /// # Intuition
    /// We don't need to fully sort the array. We only need to find the (n-k)-th smallest
    /// element (the threshold) and count elements strictly less than it. Quickselect
    /// achieves this in O(n) average time.
    ///
    /// # Approach
    /// 1. Handle edge case k = 0: all elements qualify
    /// 2. Use select_nth_unstable to partition array at index n-k
    ///    - This puts the (n-k)-th smallest element in place
    ///    - Left partition contains all elements <= threshold
    /// 3. Count elements in left partition that are strictly less than threshold
    ///
    /// # Complexity
    /// - Time: O(n) average case for select_nth_unstable, O(n) to count
    /// - Space: O(1) extra space (in-place partitioning)
    pub fn count_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();

        // Edge case: k = 0 means all elements qualify
        if k == 0 {
            return n as i32;
        }

        // Partition at index n-k: elements before are <= threshold, after are >= threshold
        let (left, threshold, _) = nums.select_nth_unstable(n - k as usize);

        // Count elements strictly less than threshold
        left.iter().filter(|&&x| x < *threshold).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_elements(vec![3, 1, 2], 1), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::count_elements(vec![5, 5, 5], 2), 0);
    }

    #[test]
    fn test_k_zero() {
        // All elements qualify when k=0
        assert_eq!(Solution::count_elements(vec![1, 2, 3], 0), 3);
        assert_eq!(Solution::count_elements(vec![5, 5, 5], 0), 3);
        assert_eq!(Solution::count_elements(vec![621202893], 0), 1);
    }

    #[test]
    fn test_all_qualified() {
        // All elements except max have at least one greater element
        assert_eq!(Solution::count_elements(vec![1, 2, 3, 4], 1), 3);
    }

    #[test]
    fn test_none_qualified() {
        // k is too large
        assert_eq!(Solution::count_elements(vec![1, 2, 3], 5), 0);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(Solution::count_elements(vec![1, 2, 3, 3], 1), 2);
        assert_eq!(Solution::count_elements(vec![1, 1, 2, 2, 3, 3], 2), 4);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_elements(vec![1], 0), 1);
    }

    #[test]
    fn test_k_equals_n_minus_1() {
        // Only the minimum element qualifies when k = n - 1
        assert_eq!(Solution::count_elements(vec![1, 2, 3, 4], 3), 1);
    }

    #[test]
    fn test_all_unique_descending() {
        // Strictly decreasing values
        assert_eq!(Solution::count_elements(vec![4, 3, 2, 1], 2), 2);
    }

    #[test]
    fn test_large_k() {
        // k close to n
        assert_eq!(Solution::count_elements(vec![1, 2, 3, 4, 5], 4), 1);
    }

    #[test]
    fn test_two_unique_values() {
        assert_eq!(Solution::count_elements(vec![1, 1, 1, 2, 2], 2), 3);
        // Explanation: 1 has 2 elements greater (both 2s), 2 has 0 greater
    }

    #[test]
    fn test_large_gap() {
        // Large gap between values
        assert_eq!(Solution::count_elements(vec![1, 100, 100, 100], 2), 1);
    }

    #[test]
    fn test_boundary_with_duplicates() {
        // threshold is part of a duplicate group
        assert_eq!(Solution::count_elements(vec![1, 2, 2, 3], 2), 1);
        // n-k = 2, threshold = 2, only 1 qualifies
    }
}
