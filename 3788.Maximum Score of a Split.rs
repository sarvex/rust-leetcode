impl Solution {
    /// O(1) space: compute total sum, then derive prefix on-the-fly from suffix.
    ///
    /// # Intuition
    /// For split i, score = prefix_sum[i] - suffix_min[i]. Since prefix_sum[i] = total - suffix_sum,
    /// we only need running suffix_sum and suffix_min—no arrays required.
    ///
    /// # Approach
    /// First pass: compute total sum. Second pass (backward): maintain suffix_sum and suffix_min.
    /// At each split i, prefix_sum = total - suffix_sum. Then extend suffix to include nums[i].
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn maximum_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut total = 0i64;
        for &x in &nums {
            total += x as i64;
        }

        let mut suffix_sum = nums[n - 1] as i64;
        let mut suffix_min = nums[n - 1] as i64;
        let mut ans = i64::MIN;

        for i in (0..n - 1).rev() {
            ans = ans.max(total - suffix_sum - suffix_min);
            suffix_sum += nums[i] as i64;
            suffix_min = suffix_min.min(nums[i] as i64);
        }

        ans
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
