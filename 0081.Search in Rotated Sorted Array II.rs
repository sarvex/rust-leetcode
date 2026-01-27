impl Solution {
    /// Modified binary search with duplicate handling in rotated sorted array.
    ///
    /// # Intuition
    /// Same logic as the no-duplicates variant, but when `nums[mid] == nums[right]`,
    /// we cannot determine which half is sorted. Shrinking `right` by one
    /// resolves the ambiguity at the cost of O(n) worst case.
    ///
    /// # Approach
    /// Binary search with three cases: if `nums[mid] > nums[right]`, the
    /// left half is sorted; if less, the right half is sorted; if equal,
    /// decrement `right`. Within the sorted half, check if the target lies
    /// there to narrow the search.
    ///
    /// # Complexity
    /// - Time: O(log n) average, O(n) worst case (all duplicates)
    /// - Space: O(1) — iterative with scalar pointers
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                if nums[left] <= target && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid] < nums[right] {
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                right -= 1;
            }
        }

        nums[left] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_with_duplicates() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn not_found_with_duplicates() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn all_same() {
        assert!(Solution::search(vec![1, 1, 1], 1));
    }

    #[test]
    fn single_element() {
        assert!(Solution::search(vec![1], 1));
    }

    #[test]
    fn not_found_single() {
        assert!(!Solution::search(vec![1], 0));
    }
}
