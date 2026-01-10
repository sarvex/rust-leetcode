/// Dynamic programming solution for finding maximum dot product of two subsequences.
///
/// # Intuition
/// We need to select non-empty subsequences from both arrays such that their dot product
/// is maximized. Since subsequences maintain relative order, we can use dynamic programming
/// where we decide for each pair of elements whether to include them in our dot product.
///
/// # Approach
/// Define `dp[i][j]` as the maximum dot product using elements from `nums1[0..i]` and
/// `nums2[0..j]`. For each pair `(i, j)`, we have three choices:
/// 1. Skip `nums1[i-1]`: take `dp[i-1][j]`
/// 2. Skip `nums2[j-1]`: take `dp[i][j-1]`
/// 3. Include both elements: `max(dp[i-1][j-1], 0) + nums1[i-1] * nums2[j-1]`
///
/// We use `max(dp[i-1][j-1], 0)` because we can start a fresh subsequence if the previous
/// best was negative, but we must include at least one pair (hence using the product directly).
///
/// # Complexity
/// - Time: O(m × n) where m and n are lengths of the input arrays
/// - Space: O(m × n) for the DP table
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();

        // dp[i][j] represents max dot product using nums1[0..i] and nums2[0..j]
        // Initialize with MIN to handle the constraint that subsequences must be non-empty
        let mut dp = vec![vec![i32::MIN; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                let product = nums1[i - 1] * nums2[j - 1];

                // Option 1 & 2: Skip current element from either array
                let skip_current = dp[i - 1][j].max(dp[i][j - 1]);

                // Option 3: Include both current elements
                // Use max(0, previous) to allow starting fresh if previous was negative
                let include_pair = dp[i - 1][j - 1].max(0) + product;

                dp[i][j] = skip_current.max(include_pair);
            }
        }

        dp[len1][len2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_product() {
        let nums1 = vec![2, 1, -2, 5];
        let nums2 = vec![3, 0, -6];
        assert_eq!(Solution::max_dot_product(nums1, nums2), 18);
    }

    #[test]
    fn test_single_pair() {
        let nums1 = vec![3, -2];
        let nums2 = vec![2, -6, 7];
        assert_eq!(Solution::max_dot_product(nums1, nums2), 21);
    }

    #[test]
    fn test_all_negative_vs_positive() {
        let nums1 = vec![-1, -1];
        let nums2 = vec![1, 1];
        // Best we can do is minimize the negative: -1 * 1 = -1
        assert_eq!(Solution::max_dot_product(nums1, nums2), -1);
    }

    #[test]
    fn test_single_elements() {
        let nums1 = vec![5];
        let nums2 = vec![4];
        assert_eq!(Solution::max_dot_product(nums1, nums2), 20);
    }

    #[test]
    fn test_mixed_values() {
        let nums1 = vec![-3, -8, 3, -10, 1, 3, 9];
        let nums2 = vec![9, 2, 3, 7, -9, 1, -8, 5, -1, -1];
        assert_eq!(Solution::max_dot_product(nums1, nums2), 200);
    }
}
