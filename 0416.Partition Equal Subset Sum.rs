impl Solution {
    /// Determines if the array can be partitioned into two equal-sum subsets using DP.
    ///
    /// # Intuition
    /// This reduces to the 0/1 knapsack problem: can we select elements summing
    /// to exactly half the total? A 1-D boolean DP array suffices.
    ///
    /// # Approach
    /// 1. If the total sum is odd, return false immediately.
    /// 2. Create a DP array of size target + 1, with dp[0] = true.
    /// 3. For each number, update dp in reverse to avoid using the same element twice.
    ///
    /// # Complexity
    /// - Time: O(n × target)
    /// - Space: O(target)
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for &num in &nums {
            let v = num as usize;
            for j in (v..=target).rev() {
                dp[j] = dp[j] || dp[j - v];
            }
        }
        dp[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn test_cannot_partition() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn test_single_element() {
        assert!(!Solution::can_partition(vec![1]));
    }
}
