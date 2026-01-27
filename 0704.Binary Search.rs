impl Solution {
    /// Standard binary search on a sorted array.
    ///
    /// # Intuition
    /// A sorted array allows halving the search space at each step by comparing
    /// the middle element against the target.
    ///
    /// # Approach
    /// Maintain two pointers `lo` and `hi`. Compute mid, narrow the range based
    /// on comparison with target. Return index if found, otherwise -1.
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0usize, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_found() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_target_not_found() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_single_element_found() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }

    #[test]
    fn test_single_element_not_found() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn test_first_element() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 1), 0);
    }

    #[test]
    fn test_last_element() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 5), 4);
    }
}
