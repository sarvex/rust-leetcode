impl Solution {
    /// Finds the minimum common value using binary search on the larger array.
    ///
    /// # Intuition
    /// Both arrays are sorted. Iterating the smaller array and binary-searching
    /// each element in the larger one is O(m log n), which beats the O(m + n)
    /// two-pointer approach when one array is significantly shorter.
    ///
    /// # Approach
    /// 1. Ensure `nums1` is the shorter array (swap if needed).
    /// 2. For each element in `nums1`, binary-search `nums2`.
    /// 3. Return the first hit, or -1 if none found.
    ///
    /// # Complexity
    /// - Time: O(m log n) where m = min(len1, len2), n = max(len1, len2)
    /// - Space: O(1)
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (small, large) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        small
            .iter()
            .find(|&&v| large.binary_search(&v).is_ok())
            .copied()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_exists() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }

    #[test]
    fn test_no_common() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![4, 5, 6]), -1);
    }

    #[test]
    fn test_first_element_match() {
        assert_eq!(Solution::get_common(vec![1, 2], vec![1, 3]), 1);
    }

    #[test]
    fn test_single_elements() {
        assert_eq!(Solution::get_common(vec![5], vec![5]), 5);
    }

    #[test]
    fn test_minimum_common_returned() {
        assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
    }

    #[test]
    fn test_large_disjoint_prefix() {
        // nums1 is tiny, nums2 is large — binary search shines here
        assert_eq!(
            Solution::get_common(vec![100], vec![1, 2, 3, 99, 100, 101]),
            100
        );
    }
}
