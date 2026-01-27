impl Solution {
    /// Modified binary search on a rotated sorted array.
    ///
    /// # Intuition
    /// A rotated sorted array has one sorted half at every midpoint. By
    /// identifying which half is sorted, we determine whether the target
    /// lies within that half, narrowing the search by half each iteration.
    ///
    /// # Approach
    /// Standard binary search with an additional check: if the left half
    /// is sorted (`nums[left] <= nums[mid]`), check if the target falls
    /// within `[left, mid)`. Otherwise the right half is sorted, so check
    /// if the target falls within `(mid, right]`. Adjust bounds accordingly.
    ///
    /// # Complexity
    /// - Time: O(log n) — standard binary search
    /// - Space: O(1) — iterative with scalar pointers
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0usize, nums.len() - 1);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                if target >= nums[left] && target < nums[mid] {
                    right = mid.wrapping_sub(1);
                } else {
                    left = mid + 1;
                }
            } else if target > nums[mid] && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid.wrapping_sub(1);
            }

            if right == usize::MAX {
                break;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_in_left_half() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn not_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn single_element_found() {
        assert_eq!(Solution::search(vec![1], 1), 0);
    }

    #[test]
    fn single_element_not_found() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn no_rotation() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 3), 2);
    }
}
