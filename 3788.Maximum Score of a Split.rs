impl Solution {
    /// Compute prefix sum on-the-fly while iterating, using precomputed suffix minimums.
    ///
    /// # Intuition
    /// For each split index i, the score is prefix sum up to i minus the minimum in the suffix.
    /// We only need to store suffix minimums; prefix sums can be computed incrementally.
    ///
    /// # Approach
    /// First pass (backward): build suffix_min where `suffix_min[i]` is min of `nums[i+1..n]`.
    /// Second pass (forward): accumulate prefix sum and track maximum score in a single sweep.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for suffix_min array
    pub fn maximum_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut suffix_min = vec![0i64; n];
        suffix_min[n - 1] = nums[n - 1] as i64;
        for i in (0..n - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i + 1] as i64);
        }

        let mut prefix_sum = 0i64;
        let mut max_score = i64::MIN;
        for i in 0..n - 1 {
            prefix_sum += nums[i] as i64;
            max_score = max_score.max(prefix_sum - suffix_min[i]);
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::maximum_score(vec![10, -1, 3, -4, -5]),
            17
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::maximum_score(vec![-7, -5, 3]), -2);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::maximum_score(vec![1, 1]), 0);
    }

    #[test]
    fn test_two_positive() {
        assert_eq!(Solution::maximum_score(vec![5, 3]), 5 - 3);
    }

    #[test]
    fn test_negative_suffix_min_increases_score() {
        assert_eq!(Solution::maximum_score(vec![1, 2, -10]), 3 - (-10));
    }
}
