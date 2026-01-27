impl Solution {
    /// Binary search partition for median of two sorted arrays.
    ///
    /// # Intuition
    /// The median splits the combined sorted sequence into two equal halves.
    /// Binary search on the shorter array finds the correct partition point
    /// where all left-half elements are less than or equal to all right-half elements.
    ///
    /// # Approach
    /// Ensure `nums1` is the shorter array. Binary search for a partition in
    /// `nums1`; the corresponding partition in `nums2` is derived from the
    /// total half-length. Validate that `max_left_x <= min_right_y` and vice
    /// versa. Compute the median from the boundary elements based on parity.
    ///
    /// # Complexity
    /// - Time: O(log(min(m, n))) — binary search on the shorter array
    /// - Space: O(1) — only scalar variables
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let (m, n) = (nums1.len(), nums2.len());
        let half_length = (m + n + 1) / 2;
        let (mut left, mut right) = (0, m);

        while left <= right {
            let partition_x = (left + right) / 2;
            let partition_y = half_length - partition_x;

            let max_left_x = if partition_x == 0 {
                i32::MIN
            } else {
                nums1[partition_x - 1]
            };
            let min_right_x = if partition_x == m {
                i32::MAX
            } else {
                nums1[partition_x]
            };
            let max_left_y = if partition_y == 0 {
                i32::MIN
            } else {
                nums2[partition_y - 1]
            };
            let min_right_y = if partition_y == n {
                i32::MAX
            } else {
                nums2[partition_y]
            };

            if max_left_x <= min_right_y && max_left_y <= min_right_x {
                return if (m + n) % 2 == 1 {
                    max_left_x.max(max_left_y) as f64
                } else {
                    (max_left_x.max(max_left_y) as f64 + min_right_x.min(min_right_y) as f64) / 2.0
                };
            } else if max_left_x > min_right_y {
                right = partition_x - 1;
            } else {
                left = partition_x + 1;
            }
        }

        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_total_length() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn even_total_length() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn one_empty_array() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    }

    #[test]
    fn identical_values() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1], vec![1, 1]),
            1.0
        );
    }

    #[test]
    fn single_elements() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![2]), 1.5);
    }
}
