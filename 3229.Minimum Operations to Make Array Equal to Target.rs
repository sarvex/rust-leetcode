impl Solution {
    /// Computes the minimum number of range ±1 operations to match `target`.
    ///
    /// # Intuition
    /// The difference array `diff[i] = target[i] - nums[i]` represents how much each
    /// position must change. A range operation adds either +1 or -1 on a contiguous
    /// segment of `diff`, so positive and negative requirements can be handled
    /// independently without canceling each other.
    ///
    /// # Approach
    /// - Split `diff` into `pos[i] = max(diff[i], 0)` and `neg[i] = max(-diff[i], 0)`.
    /// - For a non-negative array, the minimum number of range +1 operations equals
    ///   the sum of increases between adjacent elements: start a new segment whenever
    ///   the required height rises.
    /// - Apply this to both `pos` and `neg`, and add the results.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut operations = 0i64;
        let mut prev_pos = 0i64;
        let mut prev_neg = 0i64;

        for (&num, &tar) in nums.iter().zip(target.iter()) {
            let diff = tar as i64 - num as i64;
            let pos = diff.max(0);
            let neg = (-diff).max(0);
            if pos > prev_pos {
                operations += pos - prev_pos;
            }
            if neg > prev_neg {
                operations += neg - prev_neg;
            }
            prev_pos = pos;
            prev_neg = neg;
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        let nums = vec![3, 5, 1, 2];
        let target = vec![4, 6, 2, 4];
        assert_eq!(Solution::minimum_operations(nums, target), 2);
    }

    #[test]
    fn test_example_two() {
        let nums = vec![1, 3, 2];
        let target = vec![2, 1, 4];
        assert_eq!(Solution::minimum_operations(nums, target), 5);
    }

    #[test]
    fn test_already_equal() {
        let nums = vec![2, 2, 2];
        let target = vec![2, 2, 2];
        assert_eq!(Solution::minimum_operations(nums, target), 0);
    }

    #[test]
    fn test_alternating_deltas() {
        let nums = vec![2, 2, 2, 2];
        let target = vec![3, 1, 3, 1];
        assert_eq!(Solution::minimum_operations(nums, target), 4);
    }

    #[test]
    fn test_single_large_gap() {
        let nums = vec![1];
        let target = vec![100_000_000];
        assert_eq!(Solution::minimum_operations(nums, target), 99_999_999);
    }
}
