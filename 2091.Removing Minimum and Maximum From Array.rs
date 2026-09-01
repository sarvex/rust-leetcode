impl Solution {
    /// Find the minimum deletions to remove both the min and max elements.
    ///
    /// # Intuition
    /// Elements can only be removed from the front or the back. To remove both
    /// the minimum and maximum, we only care about their positions. There are
    /// three strategies: remove both from the front, remove both from the back,
    /// or remove one from the front and the other from the back.
    ///
    /// # Approach
    /// Locate the indices `i` and `j` of the minimum and maximum values in a
    /// single pass, then let `lo = min(i, j)` and `hi = max(i, j)`. The minimum
    /// deletions is the smallest of:
    /// - Both from front: `hi + 1`
    /// - Both from back: `n - lo`
    /// - One from each end: `(lo + 1) + (n - hi)`
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut min_idx, mut max_idx) = (0usize, 0usize);
        for (i, &v) in nums.iter().enumerate() {
            if v < nums[min_idx] {
                min_idx = i;
            }
            if v > nums[max_idx] {
                max_idx = i;
            }
        }

        let lo = min_idx.min(max_idx);
        let hi = min_idx.max(max_idx);

        let both_front = hi + 1;
        let both_back = n - lo;
        let split = (lo + 1) + (n - hi);

        both_front.min(both_back).min(split) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]),
            5
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]),
            3
        );
    }

    #[test]
    fn test_example_3_single_element() {
        assert_eq!(Solution::minimum_deletions(vec![101]), 1);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::minimum_deletions(vec![1, 2]), 2);
        assert_eq!(Solution::minimum_deletions(vec![2, 1]), 2);
    }

    #[test]
    fn test_min_and_max_at_ends() {
        // min at front (index 0), max at back (index 4)
        assert_eq!(Solution::minimum_deletions(vec![-5, 3, 4, 2, 9]), 2);
        // max at front, min at back
        assert_eq!(Solution::minimum_deletions(vec![9, 3, 4, 2, -5]), 2);
    }

    #[test]
    fn test_both_near_front() {
        // max at 0, min at 1 -> both from front = 2
        assert_eq!(Solution::minimum_deletions(vec![10, 1, 5, 6, 7]), 2);
    }

    #[test]
    fn test_both_near_back() {
        // min at n-2, max at n-1 -> both from back = 2
        assert_eq!(Solution::minimum_deletions(vec![5, 6, 7, 1, 10]), 2);
    }

    #[test]
    fn test_boundary_values() {
        let nums = vec![100_000, 0, -100_000];
        // max at 0, min at 2 -> split: min(3, 3, (1)+(1)=... ) both-front=3, both-back=3, split=(0+1)+(3-2)=2
        assert_eq!(Solution::minimum_deletions(nums), 2);
    }
}
