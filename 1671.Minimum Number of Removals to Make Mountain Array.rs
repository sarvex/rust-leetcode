impl Solution {
    /// LIS from both directions to find longest mountain subsequence.
    ///
    /// # Intuition
    /// A mountain array has a peak with strictly increasing left side and
    /// strictly decreasing right side. Computing LIS from the left and LDS
    /// (LIS from the right) for each position, the longest mountain through
    /// position i has length `left[i] + right[i] - 1`. Minimize removals
    /// as `n - max_mountain_length`.
    ///
    /// # Approach
    /// 1. Compute `left[i]` = LIS length ending at i (left to right)
    /// 2. Compute `right[i]` = LIS length ending at i (right to left)
    /// 3. Find max `left[i] + right[i] - 1` where both > 1
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n)
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    left[i] = left[i].max(left[j] + 1);
                }
            }
        }

        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if nums[i] > nums[j] {
                    right[i] = right[i].max(right[j] + 1);
                }
            }
        }

        let max_mountain = (0..n)
            .filter(|&i| left[i] > 1 && right[i] > 1)
            .map(|i| left[i] + right[i] - 1)
            .max()
            .unwrap_or(0);

        n as i32 - max_mountain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_mountain() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
    }

    #[test]
    fn remove_elements() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }

    #[test]
    fn longer_array() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]),
            4
        );
    }
}
