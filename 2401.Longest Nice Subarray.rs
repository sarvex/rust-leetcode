impl Solution {
    /// Finds the longest subarray where all pairs have bitwise AND equal to zero.
    ///
    /// # Intuition
    /// A "nice" subarray requires no two elements share any set bit. We can track
    /// the union of all set bits with a bitmask and use a sliding window to maintain
    /// the invariant.
    ///
    /// # Approach
    /// 1. Maintain a sliding window with left pointer `l` and iterate right pointer `r`
    /// 2. Track `mask` as the OR of all elements in the current window
    /// 3. When a new element conflicts (mask & x != 0), shrink from the left using XOR
    /// 4. Update the answer with the current window size
    ///
    /// # Complexity
    /// - Time: O(n) — each element is added and removed at most once
    /// - Space: O(1) — only scalar variables used
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mask = 0;
        let mut l = 0;
        for (r, &x) in nums.iter().enumerate() {
            while mask & x != 0 {
                mask ^= nums[l];
                l += 1;
            }
            mask |= x;
            ans = ans.max((r - l + 1) as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::longest_nice_subarray(vec![42]), 1);
    }

    #[test]
    fn test_all_non_overlapping_bits() {
        assert_eq!(Solution::longest_nice_subarray(vec![1, 2, 4, 8]), 4);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_nice_subarray(vec![7, 7, 7]), 1);
    }
}
