impl Solution {
    /// Finds the minimum element in a rotated sorted array using binary search.
    ///
    /// # Intuition
    /// In a rotated sorted array, the minimum lies at the rotation pivot.
    /// Comparing the midpoint with the rightmost element determines which
    /// half contains the minimum.
    ///
    /// # Approach
    /// 1. Binary search with `left` and `right` pointers.
    /// 2. If `nums[mid] > nums[right]`, the minimum is in the right half.
    /// 3. Otherwise, the minimum is at or left of mid.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotated_array() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn fully_rotated() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn no_rotation() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_min(vec![1]), 1);
    }
}
