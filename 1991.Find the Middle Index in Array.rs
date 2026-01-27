impl Solution {
    /// Finds the leftmost index where left sum equals right sum.
    ///
    /// # Intuition
    /// Maintain running left and right sums. At each index, the right sum
    /// excludes the current element; check equality before adding to the left.
    ///
    /// # Approach
    /// 1. Compute total sum as the initial right sum.
    /// 2. For each index, subtract the element from right.
    /// 3. If left equals right, return the index.
    /// 4. Add the element to left.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right: i32 = nums.iter().sum();

        for (i, &val) in nums.iter().enumerate() {
            right -= val;
            if left == right {
                return i as i32;
            }
            left += val;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_exists() {
        assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    }

    #[test]
    fn test_first_index() {
        assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
    }

    #[test]
    fn test_no_middle() {
        assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
    }
}
