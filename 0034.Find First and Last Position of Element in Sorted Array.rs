impl Solution {
    /// Double binary search for first and last position of target in sorted array.
    ///
    /// # Intuition
    /// The leftmost occurrence is the lower bound of the target, and the
    /// rightmost is one less than the lower bound of `target + 1`. Two
    /// binary searches with the same helper achieve this cleanly.
    ///
    /// # Approach
    /// Define a `lower_bound` closure that finds the first index where
    /// `nums[index] >= x`. Call it for `target` (left boundary) and
    /// `target + 1` (right boundary). If no element equals the target,
    /// return `[-1, -1]`.
    ///
    /// # Complexity
    /// - Time: O(log n) — two binary searches
    /// - Space: O(1) — iterative with scalar pointers
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower_bound = |x: i32| -> usize {
            let (mut left, mut right) = (0, nums.len());
            while left < right {
                let mid = left + (right - left) / 2;
                if nums[mid] < x {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        };

        let left = lower_bound(target);
        let right = lower_bound(target + 1);

        if left == right {
            vec![-1, -1]
        } else {
            vec![left as i32, (right - 1) as i32]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_with_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn target_not_found() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn empty_array() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn single_occurrence() {
        assert_eq!(Solution::search_range(vec![1, 2, 3], 2), vec![1, 1]);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::search_range(vec![1, 1, 1], 1), vec![0, 2]);
    }
}
