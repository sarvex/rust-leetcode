impl Solution {
    /// Finds the length of the longest increasing subsequence using DP.
    ///
    /// # Intuition
    /// For each element, the LIS ending there equals 1 plus the maximum LIS
    /// of any smaller preceding element.
    ///
    /// # Approach
    /// 1. Initialize a DP array with all ones.
    /// 2. For each element, check all previous elements.
    /// 3. If a previous element is smaller, update dp[i] = max(dp[i], dp[j] + 1).
    /// 4. Return the maximum value in dp.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        for i in 1..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn all_increasing() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7]), 1);
    }
}
