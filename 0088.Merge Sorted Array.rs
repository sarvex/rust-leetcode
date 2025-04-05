impl Solution {
    /// Merges two sorted arrays nums1 and nums2 into a single sorted array stored in nums1.
    ///
    /// # intuition
    /// Start from the end of both arrays and place the larger element at the end of nums1.
    /// This avoids having to create a temporary array or shift elements.
    ///
    /// # approach
    /// 1. Initialize pointers to the last elements of both arrays and the last position in nums1
    /// 2. Compare elements from both arrays and place the larger one at the current position in nums1
    /// 3. Move the pointers accordingly
    /// 4. Continue until all elements from nums2 are placed in nums1
    ///
    /// # complexity
    /// - Time complexity: O(m+n) where m and n are the lengths of nums1 and nums2
    /// - Space complexity: O(1) as we use constant extra space
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m: usize = m as usize;
        let mut n: usize = n as usize;

        if n == 0 {
            return;
        }

        let mut index: usize = m + n - 1;

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[index] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[index] = nums2[n - 1];
                n -= 1;
            }
            index -= 1;
        }
    }
}
