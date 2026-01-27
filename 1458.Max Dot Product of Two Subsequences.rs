impl Solution {
    /// 2D DP for maximum dot product of two subsequences.
    ///
    /// # Intuition
    /// For each pair (i, j), decide whether to include both elements in the
    /// dot product. The DP state `dp[i][j]` tracks the best dot product using
    /// prefixes `nums1[0..i]` and `nums2[0..j]`. Starting fresh is allowed
    /// by clamping the previous state to zero.
    ///
    /// # Approach
    /// 1. Initialize DP table with `i32::MIN`
    /// 2. For each (i, j): consider skipping either element or including both
    /// 3. Including both: `max(dp[i-1][j-1], 0) + nums1[i-1] * nums2[j-1]`
    /// 4. Return `dp[m][n]`
    ///
    /// # Complexity
    /// - Time: O(m × n)
    /// - Space: O(m × n)
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![i32::MIN; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                let product = nums1[i - 1] * nums2[j - 1];
                let skip = dp[i - 1][j].max(dp[i][j - 1]);
                let include = dp[i - 1][j - 1].max(0) + product;
                dp[i][j] = skip.max(include);
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_product() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
    }

    #[test]
    fn single_pair() {
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
    }

    #[test]
    fn all_negative_vs_positive() {
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
    }

    #[test]
    fn single_elements() {
        assert_eq!(Solution::max_dot_product(vec![5], vec![4]), 20);
    }
}
