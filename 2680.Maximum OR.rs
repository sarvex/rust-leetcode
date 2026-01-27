impl Solution {
    /// Maximize the OR of the array by applying k left-shifts to one element.
    ///
    /// # Intuition
    /// Shifting one element left by k bits maximizes its contribution. Use prefix
    /// and suffix OR arrays to compute the OR excluding each element, then try
    /// shifting each element.
    ///
    /// # Approach
    /// 1. Build suffix OR array
    /// 2. Iterate with running prefix OR
    /// 3. For each element, compute prefix_or | (element << k) | suffix_or
    /// 4. Return the maximum
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut suffix = vec![0i64; n + 1];

        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] | (nums[i] as i64);
        }

        let mut ans = 0i64;
        let mut prefix = 0i64;
        let k64 = k as i64;

        for i in 0..n {
            ans = ans.max(prefix | ((nums[i] as i64) << k64) | suffix[i + 1]);
            prefix |= nums[i] as i64;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::maximum_or(vec![12, 9], 1), 30);
    }

    #[test]
    fn test_multiple_shifts() {
        assert_eq!(Solution::maximum_or(vec![2, 1, 8], 2), 35);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximum_or(vec![4], 3), 32);
    }
}
