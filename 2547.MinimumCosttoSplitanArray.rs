impl Solution {
    /// Minimum Cost to Split an Array
    ///
    /// # Intuition
    /// This is a classic partition DP problem where we need to find the optimal way
    /// to split an array to minimize total cost. The cost of each subarray depends
    /// on elements that appear more than once (the "trimmed" version).
    ///
    /// # Approach
    /// - Use dynamic programming where `dp[i]` represents the minimum cost to split `nums[0..i]`
    /// - For each position `i`, try all possible last subarrays ending at index `i-1`
    /// - Incrementally compute the trimmed length by tracking frequencies as we extend backward
    /// - Trimmed length = sum of frequencies for elements appearing >= 2 times
    ///
    /// # Complexity
    /// - Time: O(n²) where n is the length of nums
    /// - Space: O(n) for the DP array and frequency counter
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;

        // dp[i] = minimum cost to split nums[0..i]
        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            // Frequency counter for elements in current subarray
            // Constraint: 0 <= nums[i] < nums.length, so we can use array
            let mut freq = vec![0i32; n];
            let mut trimmed_len = 0i64;

            // Try all subarrays [j..i) ending at position i-1
            // Iterate backward to build frequency incrementally
            for j in (0..i).rev() {
                let val = nums[j] as usize;
                freq[val] += 1;

                // Update trimmed length based on new frequency
                match freq[val] {
                    2 => trimmed_len += 2, // Element now contributes (both occurrences)
                    count if count > 2 => trimmed_len += 1, // One more occurrence
                    _ => {}                // First occurrence, doesn't contribute to trimmed
                }

                // Calculate cost: previous best + importance of subarray [j..i)
                let importance = k + trimmed_len;
                dp[i] = dp[i].min(dp[j].saturating_add(importance));
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Split: [1,2], [1,2,1,3,3]
        // [1,2]: trimmed=[], importance = 2+0 = 2
        // [1,2,1,3,3]: trimmed=[1,1,3,3], importance = 2+4 = 6
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1, 3, 3], 2), 8);
    }

    #[test]
    fn test_example_2() {
        // Split: [1,2], [1,2,1]
        // [1,2]: importance = 2+0 = 2
        // [1,2,1]: trimmed=[1,1], importance = 2+2 = 4
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 2), 6);
    }

    #[test]
    fn test_example_3() {
        // Single subarray is optimal when k is large
        // [1,2,1,2,1]: trimmed=[1,2,1,2,1] (all appear >1), importance = 5+5 = 10
        assert_eq!(Solution::min_cost(vec![1, 2, 1, 2, 1], 5), 10);
    }

    #[test]
    fn test_single_element() {
        // Single element, trimmed=[], importance = k+0
        assert_eq!(Solution::min_cost(vec![0], 1), 1);
    }

    #[test]
    fn test_all_same_elements() {
        // [0,0,0]: trimmed=[0,0,0], importance = 1+3 = 4
        assert_eq!(Solution::min_cost(vec![0, 0, 0], 1), 4);
    }

    #[test]
    fn test_all_unique() {
        // All unique elements, best is to split into individual elements if k is small
        // Or keep together if k is large
        // [0,1,2]: each unique, trimmed=[] for any subarray
        // Split all: 3 * 2 = 6
        // Keep together: 2 + 0 = 2
        assert_eq!(Solution::min_cost(vec![0, 1, 2], 2), 2);
    }

    #[test]
    fn test_large_k() {
        // When k is large, prefer fewer splits
        assert_eq!(
            Solution::min_cost(vec![0, 1, 0, 1], 1_000_000_000),
            1_000_000_004
        );
    }
}
