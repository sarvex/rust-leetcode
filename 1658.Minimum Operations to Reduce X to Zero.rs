impl Solution {
    /// Minimum end-removals whose values sum to `x`.
    ///
    /// # Intuition
    /// Every value is positive, so the removed elements are a prefix plus a
    /// suffix. Their complement is one contiguous middle subarray summing to
    /// `total - x`. The fewest removals equal `n` minus the longest such subarray.
    ///
    /// # Approach
    /// 1. Let `target = sum(nums) - x`. A negative target is impossible, and a
    ///    zero target means the whole array must be removed.
    /// 2. Slide a window over `nums`. Advance the right edge, then shrink from
    ///    the left while the window sum exceeds `target`. Positive values make
    ///    each index enter and leave the window at most once.
    /// 3. Whenever the window sum equals `target`, keep the longer length.
    /// 4. Return `n - max_length`, or `-1` when no window matches.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let target = nums.iter().sum::<i32>() - x;
        if target < 0 {
            return -1;
        }
        if target == 0 {
            return n as i32;
        }

        let mut left = 0;
        let mut window = 0;
        let mut max_len = 0;

        for (right, &value) in nums.iter().enumerate() {
            window += value;
            while window > target {
                window -= nums[left];
                left += 1;
            }
            if window == target {
                max_len = max_len.max(right - left + 1);
            }
        }

        if max_len == 0 {
            -1
        } else {
            (n - max_len) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_both_ends() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn all_elements() {
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    }

    #[test]
    fn take_all() {
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }

    #[test]
    fn single_element_matches() {
        assert_eq!(Solution::min_operations(vec![1], 1), 1);
    }

    #[test]
    fn single_element_impossible() {
        assert_eq!(Solution::min_operations(vec![2], 1), -1);
    }

    #[test]
    fn remove_entire_array() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], 6), 3);
    }

    #[test]
    fn left_prefix_shorter_than_suffix() {
        assert_eq!(Solution::min_operations(vec![1, 2, 3], 3), 1);
    }
}
