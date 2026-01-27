impl Solution {
    /// Finds the pivot index where left sum equals right sum.
    ///
    /// # Intuition
    /// Precompute the total sum, then scan left-to-right maintaining a running
    /// left sum. The right sum is `total - left - nums[i]`.
    ///
    /// # Approach
    /// Compute total sum. Iterate through the array; at each index check if
    /// left sum equals `total - left - current`. Return the first such index.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left = 0;
        for (i, &num) in nums.iter().enumerate() {
            if left == total - left - num {
                return i as i32;
            }
            left += num;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_exists() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test_no_pivot() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_pivot_at_start() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::pivot_index(vec![1]), 0);
    }
}
