
impl Solution {
    /// Finds the single non-duplicate element in a sorted array using binary search.
    ///
    /// # Intuition
    /// In a sorted array of pairs, the single element disrupts the even-odd
    /// pairing. Binary search on index parity locates the disruption.
    ///
    /// # Approach
    /// 1. Binary search with `mid ^ 1` to find the expected pair partner.
    /// 2. If `nums[mid] == nums[mid ^ 1]`, the single element is to the right.
    /// 3. Otherwise it is at `mid` or to the left.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] != nums[mid ^ 1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
    }

    #[test]
    fn test_at_end() {
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::single_non_duplicate(vec![1]), 1);
    }

    #[test]
    fn test_at_beginning() {
        assert_eq!(Solution::single_non_duplicate(vec![1, 2, 2, 3, 3, 4, 4]), 1);
    }

    #[test]
    fn test_at_very_end() {
        assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 2, 3, 3, 4]), 4);
    }

    #[test]
    fn test_three_elements() {
        assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2]), 2);
        assert_eq!(Solution::single_non_duplicate(vec![1, 2, 2]), 1);
    }

    #[test]
    fn test_middle() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 2, 3, 4, 4, 5, 5]),
            3
        );
    }
}
