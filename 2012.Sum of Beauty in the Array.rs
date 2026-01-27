impl Solution {
    /// Computes the sum of beauty values for each interior element.
    ///
    /// # Intuition
    /// An element gets beauty 2 if it is strictly between the prefix max
    /// and suffix min, or beauty 1 if only between its immediate neighbors.
    ///
    /// # Approach
    /// 1. Build a suffix minimum array from right to left.
    /// 2. Track prefix maximum from left to right.
    /// 3. For each interior index, check both conditions.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suffix_min = vec![0; n];
        suffix_min[n - 1] = nums[n - 1];
        for i in (1..n - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i]);
        }

        let mut prefix_max = nums[0];
        let mut result = 0;

        for i in 1..n - 1 {
            if prefix_max < nums[i] && nums[i] < suffix_min[i + 1] {
                result += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                result += 1;
            }
            prefix_max = prefix_max.max(nums[i]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
    }

    #[test]
    fn test_no_beauty() {
        assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
    }
}
