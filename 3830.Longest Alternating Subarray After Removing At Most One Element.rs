impl Solution {
    /// Computes the longest alternating subarray after removing at most one element.
    ///
    /// # Intuition
    /// An alternating subarray is a zigzag of adjacent comparisons. Removing one element allows
    /// skipping exactly one position, so we can model the best zigzag ending at each index with
    /// either no skips or one skip already used.
    ///
    /// # Approach
    /// - Track two DP states per index and last comparison direction: `up` (last step up) and
    ///   `down` (last step down).
    /// - For zero removals, use the standard zigzag recurrence from the previous index.
    /// - For one removal, either extend a sequence that already used the removal, or delete the
    ///   immediate predecessor and connect `i-2` directly to `i`.
    /// - The answer is the maximum length across all states.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn longest_alternating(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut up0 = vec![1_usize; n];
        let mut down0 = vec![1_usize; n];
        let mut up1 = vec![0_usize; n];
        let mut down1 = vec![0_usize; n];
        let mut best = 1_usize;

        for i in 1..n {
            if nums[i - 1] < nums[i] {
                up0[i] = down0[i - 1] + 1;
                down0[i] = 1;
            } else if nums[i - 1] > nums[i] {
                down0[i] = up0[i - 1] + 1;
                up0[i] = 1;
            } else {
                up0[i] = 1;
                down0[i] = 1;
            }

            if nums[i - 1] < nums[i] {
                if down1[i - 1] > 0 {
                    up1[i] = up1[i].max(down1[i - 1] + 1);
                }
            } else if nums[i - 1] > nums[i] {
                if up1[i - 1] > 0 {
                    down1[i] = down1[i].max(up1[i - 1] + 1);
                }
            }

            if i >= 2 {
                if nums[i - 2] < nums[i] {
                    up1[i] = up1[i].max(down0[i - 2] + 1);
                } else if nums[i - 2] > nums[i] {
                    down1[i] = down1[i].max(up0[i - 2] + 1);
                }
            }

            best = best.max(up0[i]).max(down0[i]).max(up1[i]).max(down1[i]);
        }

        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_alternating(vec![2, 1, 3, 2]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_alternating(vec![3, 2, 1, 2, 3, 2, 1]), 4,);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_alternating(vec![100_000, 100_000]), 1);
    }

    #[test]
    fn test_requires_deletion() {
        assert_eq!(Solution::longest_alternating(vec![1, 2, 2, 1]), 3);
    }

    #[test]
    fn test_monotonic_increasing() {
        assert_eq!(Solution::longest_alternating(vec![1, 2, 3, 4, 5]), 2);
    }
}
