impl Solution {
    /// Finds the maximum difference nums[j] - nums[i] where i < j and nums[i] < nums[j].
    ///
    /// # Intuition
    /// Track the running minimum. For each element greater than the minimum,
    /// update the best difference.
    ///
    /// # Approach
    /// 1. Maintain the minimum value seen so far.
    /// 2. For each element, if it exceeds the minimum, update the answer.
    /// 3. Otherwise, update the minimum.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min_val = i32::MAX;
        let mut result = -1;

        for &x in &nums {
            if x > min_val {
                result = result.max(x - min_val);
            } else {
                min_val = x;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
    }

    #[test]
    fn test_no_increasing_pair() {
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
    }

    #[test]
    fn test_increasing() {
        assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
