impl Solution {
    /// Binary search with duplicate handling for rotated sorted array minimum.
    ///
    /// # Intuition
    /// Like problem 153, but duplicates mean `nums[mid] == nums[right]` is ambiguous —
    /// we can't determine which half is sorted, so we shrink the right boundary by one.
    ///
    /// # Approach
    /// Maintain `left` and `right` pointers. At each step:
    /// - If `nums[mid] < nums[right]`: minimum is in `[left, mid]`, set `right = mid`.
    /// - If `nums[mid] > nums[right]`: minimum is in `[mid+1, right]`, set `left = mid + 1`.
    /// - If `nums[mid] == nums[right]`: can't tell which side; safely shrink `right -= 1`.
    ///
    /// # Complexity
    /// - Time: O(log n) average, O(n) worst case (all duplicates)
    /// - Space: O(1)
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&nums[right]) {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
                std::cmp::Ordering::Equal => right -= 1,
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duplicates_rotated() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_with_duplicates_rotated() {
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::find_min(vec![3, 3, 3, 3]), 3);
    }

    #[test]
    fn test_two_elements_rotated() {
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }

    #[test]
    fn test_duplicates_at_pivot() {
        assert_eq!(Solution::find_min(vec![3, 3, 1, 3]), 1);
    }

    #[test]
    fn test_min_at_end() {
        assert_eq!(Solution::find_min(vec![5, 5, 5, 1]), 1);
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(Solution::find_min(vec![-1, -1, 0, -1]), -1);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(Solution::find_min(vec![-5000, -5000, 5000, -5000]), -5000);
    }
}
