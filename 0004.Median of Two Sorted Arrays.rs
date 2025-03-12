/**
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *
 */
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let m = nums1.len();
        let n = nums2.len();
        let total_length = m + n;
        let half_length = (total_length + 1) / 2;

        let mut left_bound = 0;
        let mut right_bound = m;

        while left_bound <= right_bound {
            let partition_x = (left_bound + right_bound) / 2;
            let partition_y = half_length - partition_x;

            let max_left_x = if partition_x == 0 { i32::MIN } else { nums1[partition_x - 1] };
            let min_right_x = if partition_x == m { i32::MAX } else { nums1[partition_x] };
            let max_left_y = if partition_y == 0 { i32::MIN } else { nums2[partition_y - 1] };
            let min_right_y = if partition_y == n { i32::MAX } else { nums2[partition_y] };

            if max_left_x <= min_right_y && max_left_y <= min_right_x {
                if total_length % 2 == 1 {
                    return max_left_x.max(max_left_y) as f64;
                } else {
                    let max_of_left = max_left_x.max(max_left_y) as f64;
                    let min_of_right = min_right_x.min(min_right_y) as f64;
                    return (max_of_left + min_of_right) / 2.0;
                }
            } else if max_left_x > min_right_y {
                right_bound = partition_x - 1;
            } else {
                left_bound = partition_x + 1;
            }
        }

        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_empty_array() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test_same_values() {
        let nums1 = vec![1, 1];
        let nums2 = vec![1, 1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }
}
